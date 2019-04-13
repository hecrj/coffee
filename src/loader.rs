use crate::graphics;

pub trait Loader<T> {
    fn total_work(&self) -> u32;

    fn load(&mut self, gpu: &mut graphics::Gpu) -> Progress<T>;
}

pub enum Progress<T> {
    Loading { work_completed: u32 },
    Done(T),
}

impl<T, F> Loader<T> for F
where
    F: FnMut(&mut graphics::Gpu) -> T,
{
    fn total_work(&self) -> u32 {
        1
    }

    fn load(&mut self, gpu: &mut graphics::Gpu) -> Progress<T> {
        Progress::Done(self(gpu))
    }
}

pub struct Map<T> {
    total_work: u32,
    function: Box<FnMut(&mut graphics::Gpu) -> Progress<T>>,
}

impl<T> Loader<T> for Map<T> {
    fn total_work(&self) -> u32 {
        self.total_work
    }

    fn load(&mut self, gpu: &mut graphics::Gpu) -> Progress<T> {
        (self.function)(gpu)
    }
}

pub fn map<A, F, T>(
    mut loader: impl 'static + Loader<A>,
    f: F,
) -> impl Loader<T>
where
    A: 'static,
    F: 'static + Fn(A) -> T,
{
    Map {
        total_work: loader.total_work(),
        function: Box::new(move |gpu| match loader.load(gpu) {
            Progress::Loading { work_completed } => {
                Progress::Loading { work_completed }
            }
            Progress::Done(a) => Progress::Done(f(a)),
        }),
    }
}

pub fn map2<A, B, F, T>(
    mut loader_a: impl 'static + Loader<A>,
    mut loader_b: impl 'static + Loader<B>,
    f: F,
) -> Map<T>
where
    A: 'static,
    B: 'static,
    F: 'static + Fn(A, B) -> T,
{
    let mut loaded = false;

    Map {
        total_work: loader_a.total_work() + loader_b.total_work(),
        function: Box::new(move |gpu| match loader_a.load(gpu) {
            Progress::Loading { work_completed } => {
                Progress::Loading { work_completed }
            }
            Progress::Done(a) => {
                if loaded {
                    match loader_b.load(gpu) {
                        Progress::Loading { work_completed } => {
                            Progress::Loading {
                                work_completed: loader_a.total_work()
                                    + work_completed,
                            }
                        }
                        Progress::Done(b) => Progress::Done(f(a, b)),
                    }
                } else {
                    loaded = true;

                    Progress::Loading {
                        work_completed: loader_a.total_work(),
                    }
                }
            }
        }),
    }
}

pub fn map3<A, B, C, F, T>(
    loader_a: impl 'static + Loader<A>,
    loader_b: impl 'static + Loader<B>,
    loader_c: impl 'static + Loader<C>,
    f: F,
) -> Map<T>
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: 'static + Fn(A, B, C) -> T,
{
    let many = map2(loader_a, loader_b, |a, b| (a, b));

    map2(many, loader_c, move |(a, b), c| f(a, b, c))
}
