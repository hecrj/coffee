use crate::graphics;
use crate::Result;

/// A `Task<T>` represents an operation that produces a value of type `T`.
///
/// # Laziness
/// A [`Task`] is a recipe that describes how to produce a specific output,
/// like a function. They can be combined or transformed in certain ways and
/// run whenever needed.
///
/// Creating a [`Task`] consists in specifying this recipe. For instance,
/// we could define a task to load an `Image` like this:
///
/// ```
/// # use coffee::load::Task;
/// # use coffee::graphics::Image;
/// #
/// let load_image = Task::using_gpu(|gpu| Image::new(gpu, "my-image.png"));
/// ```
///
/// Here we just _describe_ how to load an image, we do not load it right away.
/// This is how [`Image::load`] works, you should use that instead of writing
/// this over and over!
///
/// # Composition
/// Tasks can be combined thanks to the [`Join`] trait.
///
/// Let's say we have a `Task<Image>` and a `Task<TextureArray>`, we can obtain
/// a `Task<(Image, TextureArray)>` like this:
///
/// ```
/// # use coffee::load::Task;
/// # let load_image = Task::succeed(|| ());
/// # let load_texture_array = Task::succeed(|| ());
/// #
/// use coffee::load::Join;
///
/// let combined_task = (load_image, load_texture_array).join();
/// ```
///
/// You can do this for up to 8 tasks. However, consider grouping task output in
/// meaningful structs using [`map`]:
///
/// ```
/// # use coffee::load::Task;
/// # use coffee::graphics::Image;
/// #
/// use coffee::load::Join;
///
/// pub struct PlayerAssets {
///     idle: Image,
///     running: Image,
/// }
///
/// impl PlayerAssets {
///     pub fn load() -> Task<PlayerAssets> {
///         (
///             Image::load("player/idle.png"),
///             Image::load("player/running.png"),
///         )
///             .join()
///             .map(|(idle, running)| PlayerAssets { idle, running })
///     }
/// }
/// ```
///
/// [`Task`]: struct.Task.html
/// [`Join`]: trait.Join.html
/// [`Image::load`]: ../graphics/struct.Image.html#method.load
/// [`map`]: #method.map
pub struct Task<T> {
    total_work: u32,
    function: Box<dyn FnOnce(&mut Worker<'_>) -> Result<T>>,
}

impl<T> Task<T> {
    /// Creates a new [`Task`] from a lazy operation.
    ///
    /// Imagine we had to generate a random game map, we could represent this
    /// as a [`Task`]:
    /// ```
    /// # use coffee::load::Task;
    /// struct Map {
    ///     // ...
    /// }
    ///
    /// impl Map {
    ///     pub fn generate() -> Map {
    ///         Map { /*...*/ }
    ///     }
    /// }
    ///
    /// let generate_map = Task::new(|| Ok(Map::generate()));
    /// ```
    ///
    /// [`Task`]: struct.Task.html
    pub fn new<F>(f: F) -> Task<T>
    where
        F: 'static + FnOnce() -> Result<T>,
    {
        Task {
            total_work: 1,
            function: Box::new(move |worker| {
                let result = f();

                worker.notify_progress(1);

                result
            }),
        }
    }

    /// Creates a new [`Task`] from a lazy operation that cannot fail.
    ///
    /// ```rust
    /// # use coffee::load::Task;
    /// struct Map {
    ///     // ...
    /// }
    ///
    /// impl Map {
    ///     pub fn generate() -> Map {
    ///         Map { /*...*/ }
    ///     }
    /// }
    ///
    /// let generate_map = Task::succeed(Map::generate);
    /// ```
    ///
    /// [`Task`]: struct.Task.html
    pub fn succeed<F>(f: F) -> Task<T>
    where
        F: 'static + FnOnce() -> T,
    {
        Task::new(move || Ok(f()))
    }

    /// Creates a new [`Task`] that uses a [`Gpu`].
    ///
    /// You can use this to load and prepare graphical assets.
    ///
    /// Keep in mind that many types in [`graphics`] already implement loading
    /// methods returning a `Task` (like [`Image::load`] or [`Font::load_from_bytes`]).
    /// Before using this, check out whether whatever you want to load has
    /// already a useful helper that suits your needs!
    ///
    /// [`Task`]: struct.Task.html
    /// [`Gpu`]: ../graphics/struct.Gpu.html
    /// [`graphics`]: ../graphics/index.html
    /// [`Task`]: struct.Task.html
    /// [`Image::load`]: ../graphics/struct.Image.html#method.load
    /// [`Font::load_from_bytes`]: ../graphics/struct.Font.html#method.load_from_bytes
    pub fn using_gpu<F>(f: F) -> Task<T>
    where
        F: 'static + FnOnce(&mut graphics::Gpu) -> Result<T>,
    {
        Task::sequence(1, move |worker| {
            let result = f(worker.gpu());

            worker.notify_progress(1);

            result
        })
    }

    pub(crate) fn sequence<F>(total_work: u32, f: F) -> Task<T>
    where
        F: 'static + FnOnce(&mut Worker<'_>) -> Result<T>,
    {
        Task {
            total_work,
            function: Box::new(f),
        }
    }

    /// Adds a title to the [`Task`].
    ///
    /// The title will be used when reporting progress once the [`Task`] is run.
    /// This allows task runners, like loading screens, to show additional
    /// feedback to the user.
    ///
    /// For example, let's say we want to generate a map and load some terrain
    /// assets. We can define a couple stages for each task:
    /// ```
    /// # use coffee::load::Task;
    /// # use coffee::graphics::Image;
    /// # struct Map;
    /// # impl Map {
    /// # fn generate() -> Map { Map }
    /// # }
    /// # struct TerrainAssets;
    /// # impl TerrainAssets {
    /// # fn load() -> Task<()> { Task::succeed(|| ()) }
    /// # }
    /// use coffee::load::Join;
    ///
    /// let load_game =
    ///     (
    ///         Task::stage("Generating map...", Task::succeed(Map::generate)),
    ///         Task::stage("Loading terrain...", TerrainAssets::load())
    ///     )
    ///         .join();
    /// ```
    /// If we then used this [`Task`] with the [`ProgressBar`] loading screen, it
    /// would show each of these titles on top of the progress bar when their
    /// according tasks are being run.
    ///
    /// [`Task`]: struct.Task.html
    /// [`ProgressBar`]: loading_screen/struct.ProgressBar.html
    pub fn stage<S: Into<String>>(title: S, task: Task<T>) -> Task<T>
    where
        T: 'static,
    {
        let title = title.into();

        Task {
            total_work: task.total_work,
            function: Box::new(move |worker| {
                worker.with_stage(title.clone(), task.function)
            }),
        }
    }

    /// Returns the total units of work of the [`Task`].
    ///
    /// [`Task`]: struct.Task.html
    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    /// Transforms the output of a [`Task`].
    ///
    /// As [explained above], use this method to make your tasks return your
    /// own custom types, enhancing composability.
    ///
    /// [`Task`]: struct.Task.html
    /// [explained above]: #composition
    pub fn map<F, A>(self, f: F) -> Task<A>
    where
        T: 'static,
        F: 'static + FnOnce(T) -> A,
    {
        Task {
            total_work: self.total_work,
            function: Box::new(move |worker| match (self.function)(worker) {
                Ok(value) => Ok(f(value)),
                Err(error) => Err(error),
            }),
        }
    }

    /// Runs a [`Task`] and obtains the produced value.
    ///
    /// [`Task`]: struct.Task.html
    pub fn run(self, gpu: &mut graphics::Gpu) -> Result<T> {
        let mut worker = Worker::Headless(gpu);

        (self.function)(&mut worker)
    }

    /// Runs a [`Task`] and obtains the produced value.
    ///
    /// You can provide a function to keep track of [`Progress`].
    ///
    /// [`Task`]: struct.Task.html
    /// [`Progress`]: struct.Progress.html
    /// [`Window`]: ../graphics/window/struct.Window.html
    /// [open an issue]: https://github.com/hecrj/coffee/issues
    pub(crate) fn run_with_window<F>(
        self,
        window: &mut graphics::Window,
        mut on_progress: F,
    ) -> Result<T>
    where
        F: FnMut(&Progress, &mut graphics::Window) -> (),
    {
        let mut worker = Worker::Windowed {
            window,
            listener: &mut on_progress,
            progress: Progress {
                total_work: self.total_work,
                work_completed: 0,
                stages: Vec::new(),
            },
        };

        worker.notify_progress(0);

        (self.function)(&mut worker)
    }
}

impl<T> std::fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task {{ total_work: {} }}", self.total_work)
    }
}

pub(crate) enum Worker<'a> {
    Headless(&'a mut graphics::Gpu),
    Windowed {
        window: &'a mut graphics::Window,
        listener: &'a mut dyn FnMut(&Progress, &mut graphics::Window) -> (),
        progress: Progress,
    },
}

impl<'a> Worker<'a> {
    pub fn gpu(&mut self) -> &mut graphics::Gpu {
        match self {
            Worker::Headless(gpu) => gpu,
            Worker::Windowed { window, .. } => window.gpu(),
        }
    }

    pub fn notify_progress(&mut self, work: u32) {
        match self {
            Worker::Headless(_) => {}
            Worker::Windowed {
                progress,
                window,
                listener,
                ..
            } => {
                progress.work_completed += work;

                listener(&progress, window);
            }
        };
    }

    pub fn with_stage<T>(
        &mut self,
        title: String,
        f: Box<dyn FnOnce(&mut Worker<'_>) -> T>,
    ) -> T {
        match self {
            Worker::Headless(_) => f(self),
            Worker::Windowed { .. } => {
                if let Worker::Windowed { progress, .. } = self {
                    progress.stages.push(title);
                }

                self.notify_progress(0);

                let result = f(self);

                if let Worker::Windowed { progress, .. } = self {
                    let _ = progress.stages.pop();
                }

                result
            }
        }
    }
}

/// The progress of a [`Task`].
///
/// [`Task`]: struct.Task.html
#[derive(Debug, Eq, PartialEq)]
pub struct Progress {
    total_work: u32,
    work_completed: u32,
    stages: Vec<String>,
}

impl Progress {
    /// Returns the total amount of work of the related [`Task`].
    ///
    /// [`Task`]: struct.Task.html
    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    /// Returns the amount of completed work of the related [`Task`].
    ///
    /// The returned value is guaranteed to be in [0, total_work].
    ///
    /// [`Task`]: struct.Task.html
    pub fn completed_work(&self) -> u32 {
        self.work_completed.min(self.total_work)
    }

    /// Returns the amount of progress of the related [`Task`] as a percentage.
    ///
    /// You can use this value directly in your loading screen.
    ///
    /// [`Task`]: struct.Task.html
    pub fn percentage(&self) -> f32 {
        self.completed_work() as f32 / self.total_work.max(1) as f32 * 100.0
    }

    /// Returns the title of the current [`Task::stage`], if there is one.
    ///
    /// You can use this to provide additional feedback to users.
    ///
    /// [`Task::state`]: struct.Task.html#method.stage
    pub fn stage(&self) -> Option<&String> {
        self.stages.last()
    }
}

/// Join multiple tasks with ease.
///
/// Learn more about how to use this trait in the [`Task`] docs.
///
/// [`Task`]: struct.Task.html#composition
pub trait Join {
    /// The resulting output of the [`Task`] after joining.
    ///
    /// [`Task`]: struct.Task.html#composition
    type Type;

    /// Joins tasks into a new one that collects the results.
    fn join(self) -> Task<Self::Type>;
}

impl<A: 'static, B: 'static> Join for (Task<A>, Task<B>) {
    type Type = (A, B);

    fn join(self) -> Task<(A, B)> {
        let (loader_a, loader_b) = self;

        Task::sequence(
            loader_a.total_work() + loader_b.total_work(),
            move |task| {
                (loader_a.function)(task)
                    .and_then(|a| (loader_b.function)(task).map(|b| (a, b)))
            },
        )
    }
}

impl<A: 'static, B: 'static, C: 'static> Join for (Task<A>, Task<B>, Task<C>) {
    type Type = (A, B, C);

    fn join(self) -> Task<(A, B, C)> {
        let (loader_a, loader_b, loader_c) = self;

        ((loader_a, loader_b).join(), loader_c)
            .join()
            .map(|((a, b), c)| (a, b, c))
    }
}

impl<A: 'static, B: 'static, C: 'static, D: 'static> Join
    for (Task<A>, Task<B>, Task<C>, Task<D>)
{
    type Type = (A, B, C, D);

    fn join(self) -> Task<(A, B, C, D)> {
        let (loader_a, loader_b, loader_c, loader_d) = self;

        ((loader_a, loader_b, loader_c).join(), loader_d)
            .join()
            .map(|((a, b, c), d)| (a, b, c, d))
    }
}

impl<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static> Join
    for (Task<A>, Task<B>, Task<C>, Task<D>, Task<E>)
{
    type Type = (A, B, C, D, E);

    fn join(self) -> Task<(A, B, C, D, E)> {
        let (loader_a, loader_b, loader_c, loader_d, loader_e) = self;

        ((loader_a, loader_b, loader_c, loader_d).join(), loader_e)
            .join()
            .map(|((a, b, c, d), e)| (a, b, c, d, e))
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
        E: 'static,
        F: 'static,
    > Join for (Task<A>, Task<B>, Task<C>, Task<D>, Task<E>, Task<F>)
{
    type Type = (A, B, C, D, E, F);

    fn join(self) -> Task<(A, B, C, D, E, F)> {
        let (loader_a, loader_b, loader_c, loader_d, loader_e, loader_f) = self;

        (
            (loader_a, loader_b, loader_c, loader_d, loader_e).join(),
            loader_f,
        )
            .join()
            .map(|((a, b, c, d, e), f)| (a, b, c, d, e, f))
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
        E: 'static,
        F: 'static,
        G: 'static,
    > Join
    for (
        Task<A>,
        Task<B>,
        Task<C>,
        Task<D>,
        Task<E>,
        Task<F>,
        Task<G>,
    )
{
    type Type = (A, B, C, D, E, F, G);

    fn join(self) -> Task<(A, B, C, D, E, F, G)> {
        let (
            loader_a,
            loader_b,
            loader_c,
            loader_d,
            loader_e,
            loader_f,
            loader_g,
        ) = self;

        (
            (loader_a, loader_b, loader_c, loader_d, loader_e, loader_f).join(),
            loader_g,
        )
            .join()
            .map(|((a, b, c, d, e, f), g)| (a, b, c, d, e, f, g))
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
        E: 'static,
        F: 'static,
        G: 'static,
        H: 'static,
    > Join
    for (
        Task<A>,
        Task<B>,
        Task<C>,
        Task<D>,
        Task<E>,
        Task<F>,
        Task<G>,
        Task<H>,
    )
{
    type Type = (A, B, C, D, E, F, G, H);

    fn join(self) -> Task<(A, B, C, D, E, F, G, H)> {
        let (
            loader_a,
            loader_b,
            loader_c,
            loader_d,
            loader_e,
            loader_f,
            loader_g,
            loader_h,
        ) = self;

        (
            (
                loader_a, loader_b, loader_c, loader_d, loader_e, loader_f,
                loader_g,
            )
                .join(),
            loader_h,
        )
            .join()
            .map(|((a, b, c, d, e, f, g), h)| (a, b, c, d, e, f, g, h))
    }
}
