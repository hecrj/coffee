//! Load your game assets with _type-safety_ and build loading screens that can
//! keep track of progress _consistently_.
//!
//! # Tasks
//! The generic [`Task`] struct represents a lazy loading operation that can
//! be combined and composed with other tasks. Most of the types representing
//! resources in Coffee have `load` functions that return a [`Task`].
//!
//! Tasks are defined declaratively in a functional style. This allows them to
//! keep track of all the work they have to complete before even executing them.
//! Read the [`Task`] docs to learn more!
//!
//! # Loading screens
//! The [`LoadingScreen`] trait allows you to implement a loading screen that is
//! compatible with any [`Task`]. Currently, Coffee includes a built-in loading
//! screen: [`ProgressBar`], which shows a simple progress bar with some text.
//!
//! [`Task`]: struct.Task.html
//! [`LoadingScreen`]: loading_screen/trait.LoadingScreen.html
//! [`ProgressBar`]: loading_screen/struct.ProgressBar.html
pub mod loading_screen;

pub use loading_screen::LoadingScreen;

use crate::graphics;

/// A `Task<T>` represents an operation that produces a value of type `T`.
///
/// # Laziness
/// A [`Task`] is just a recipe that describes how to produce a specific asset.
/// Like functions, they can be combined and run whenever needed.
///
/// Creating a [`Task`] consists in specifying this recipe. For instance,
/// we could define a task to load an `Image` like this:
///
/// ```
/// Task::using_gpu(move |gpu| Image::new(gpu, "my-image.png"))
/// ```
///
/// Here we just _describe_ how to load an image, we do not load it right away.
/// This is how [`Image::load`] works, you should use that instead of writing
/// this over and over!
///
/// # Composition
/// Tasks can be combined easily thanks to the [`Join`] trait.
///
/// Let's say we have a `Task<Image>` and a `Task<TextureArray>`, we can easily
/// obtain a `Task<(Image, TextureArray)>` like this:
///
/// ```
/// let combined_task = (load_image, load_texture_array).join();
/// ```
///
/// You can do this for up to 8 tasks. However, consider grouping task output in
/// meaningful structs using [`map`]:
///
/// ```
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
///             .map(|(idle, running)| PlayerAssets { idle, running });
///     }
/// }
/// ```
pub struct Task<T> {
    total_work: u32,
    function: Box<Fn(&mut Worker) -> T>,
}

impl<T> Task<T> {
    pub fn new<F>(f: F) -> Task<T>
    where
        F: 'static + Fn() -> T,
    {
        Task {
            total_work: 1,
            function: Box::new(move |_| f()),
        }
    }

    pub(crate) fn sequence<F>(total_work: u32, f: F) -> Task<T>
    where
        F: 'static + Fn(&mut Worker) -> T,
    {
        Task {
            total_work,
            function: Box::new(f),
        }
    }

    pub fn using_gpu<F>(f: F) -> Task<T>
    where
        F: 'static + Fn(&mut graphics::Gpu) -> T,
    {
        Task::sequence(1, move |worker| {
            let result = f(worker.gpu());

            worker.notify_progress(1);

            result
        })
    }

    pub fn stage<S: Into<String>>(title: S, task: Task<T>) -> Task<T>
    where
        T: 'static,
    {
        let title = title.into();

        Task {
            total_work: task.total_work,
            function: Box::new(move |worker| {
                worker.with_stage(title.clone(), &task.function)
            }),
        }
    }

    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    pub fn map<F, A>(self, f: F) -> Task<A>
    where
        T: 'static,
        F: 'static + Fn(T) -> A,
    {
        Task {
            total_work: self.total_work,
            function: Box::new(move |worker| f((self.function)(worker))),
        }
    }

    pub fn run<F>(self, window: &mut graphics::Window, mut on_progress: F) -> T
    where
        F: FnMut(Progress, &mut graphics::Window) -> (),
    {
        let mut worker = Worker {
            total_work: self.total_work,
            work_completed: 0,
            stages: Vec::new(),
            window,
            listener: &mut on_progress,
        };

        worker.notify_progress(0);

        (self.function)(&mut worker)
    }
}

pub(crate) struct Worker<'a> {
    total_work: u32,
    work_completed: u32,
    stages: Vec<String>,
    window: &'a mut graphics::Window,
    listener: &'a mut FnMut(Progress, &mut graphics::Window) -> (),
}

impl<'a> Worker<'a> {
    pub fn gpu(&mut self) -> &mut graphics::Gpu {
        self.window.gpu()
    }

    pub fn notify_progress(&mut self, work: u32) {
        self.work_completed += work;

        let progress = Progress {
            total_work: self.total_work,
            work_completed: self.work_completed,
            stages: &self.stages,
        };

        (self.listener)(progress, self.window);
    }

    pub fn with_stage<T>(
        &mut self,
        title: String,
        f: &Box<Fn(&mut Worker) -> T>,
    ) -> T {
        self.stages.push(title);
        self.notify_progress(0);

        let result = f(self);
        let _ = self.stages.pop();

        result
    }
}

pub struct Progress<'a> {
    total_work: u32,
    work_completed: u32,
    stages: &'a Vec<String>,
}

impl<'a> Progress<'a> {
    pub fn percentage(&self) -> f32 {
        (self.work_completed as f32 / self.total_work.max(1) as f32 * 100.0)
            .min(100.0)
    }

    pub fn current_stage(&self) -> Option<&String> {
        self.stages.last()
    }
}

pub trait Join {
    type Type;

    fn join(self) -> Task<Self::Type>;
}

impl<A: 'static, B: 'static> Join for (Task<A>, Task<B>) {
    type Type = (A, B);

    fn join(self) -> Task<(A, B)> {
        let (loader_a, loader_b) = self;

        Task::sequence(
            loader_a.total_work() + loader_b.total_work(),
            move |task| ((loader_a.function)(task), (loader_b.function)(task)),
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
