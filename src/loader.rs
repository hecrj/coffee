use crate::graphics;

pub struct Loader<T> {
    total_work: u32,
    function: Box<Fn(&mut Task) -> T>,
}

pub struct Task<'a> {
    total_work: u32,
    work_completed: u32,
    window: &'a mut graphics::Window,
    listener: &'a mut FnMut(f32, &mut graphics::Window) -> (),
}

impl<'a> Task<'a> {
    pub fn gpu(&mut self) -> &mut graphics::Gpu {
        self.window.gpu()
    }

    pub fn progress(&mut self, work: u32) {
        self.work_completed += work;

        let progress = (self.work_completed as f32
            / self.total_work.max(1) as f32
            * 100.0)
            .min(100.0);

        (self.listener)(progress, self.window);
    }
}

impl<T> Loader<T> {
    pub fn new<F>(total_work: u32, f: F) -> Loader<T>
    where
        F: 'static + Fn(&mut Task) -> T,
    {
        Loader {
            total_work,
            function: Box::new(f),
        }
    }

    pub fn one_step<F>(f: F) -> Loader<T>
    where
        F: 'static + Fn(&mut Task) -> T,
    {
        Loader::new(1, move |task| {
            let result = f(task);

            task.progress(1);

            result
        })
    }

    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    pub fn load<F>(self, window: &mut graphics::Window, mut on_progress: F) -> T
    where
        F: FnMut(f32, &mut graphics::Window) -> (),
    {
        on_progress(0.0, window);

        let mut task = Task {
            total_work: self.total_work,
            work_completed: 0,
            window,
            listener: &mut on_progress,
        };

        (self.function)(&mut task)
    }

    pub fn map<F, A>(self, f: F) -> Loader<A>
    where
        T: 'static,
        F: 'static + Fn(T) -> A,
    {
        Loader {
            total_work: self.total_work,
            function: Box::new(move |task| f((self.function)(task))),
        }
    }
}

pub fn map2<A, B, F, T>(
    loader_a: Loader<A>,
    loader_b: Loader<B>,
    f: F,
) -> Loader<T>
where
    A: 'static,
    B: 'static,
    F: 'static + Fn(A, B) -> T,
{
    Loader::new(loader_a.total_work() + loader_b.total_work(), move |task| {
        f((loader_a.function)(task), (loader_b.function)(task))
    })
}

pub fn map3<A, B, C, F, T>(
    loader_a: Loader<A>,
    loader_b: Loader<B>,
    loader_c: Loader<C>,
    f: F,
) -> Loader<T>
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: 'static + Fn(A, B, C) -> T,
{
    let many = map2(loader_a, loader_b, |a, b| (a, b));

    map2(many, loader_c, move |(a, b), c| f(a, b, c))
}
