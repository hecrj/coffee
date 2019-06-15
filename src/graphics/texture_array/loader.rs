use std::collections::VecDeque;
use std::path::PathBuf;

use super::{Builder, Index, TextureArray};
use crate::load::Task;
use crate::{Error, Result};

/// A [`TextureArray`] builder that produces a [`Task`].
///
/// You should use [`add`] to get an index [`Key`] per texture so you can
/// retrieve each [`Index`] from the provided [`Indices`] on [`finish`].
///
/// For example, let's say that we want to use a [`TextureArray`] for our
/// entities. We could write in our `entity` module:
///
/// ```
/// use coffee::load::Task;
/// use coffee::graphics::texture_array::{TextureArray, Index, Loader};
///
/// pub struct Assets {
///     player: Index,
///     enemy: Index,
///     building: Index,
///     items: Index,
///     texture: TextureArray,
/// }
///
/// impl Assets {
///     pub fn load() -> Task<Assets> {
///         let mut loader = Loader::new(2048, 2048);
///
///         let player = loader.add("player.png");
///         let enemy = loader.add("enemy.png");
///         let building = loader.add("building.png");
///         let items = loader.add("items.png");
///
///         loader.finish(move |texture, indices| Ok(Assets {
///             player: indices.get(player)?,
///             enemy: indices.get(enemy)?,
///             building: indices.get(building)?,
///             items: indices.get(items)?,
///             texture,
///         }))
///     }
/// }
/// ```
///
/// [`TextureArray`]: struct.TextureArray.html
/// [`Task`]: ../../load/struct.Task.html
/// [`add`]: #method.add
/// [`Key`]: struct.Key.html
/// [`Index`]: struct.Index.html
/// [`Indices`]: struct.Indices.html
/// [`finish`]: #method.finish
#[derive(Debug)]
pub struct Loader {
    width: u16,
    height: u16,
    paths: Vec<PathBuf>,
}

impl Loader {
    /// Creates a new [`Loader`] that produces a [`TextureArray`] of the given
    /// size.
    ///
    /// [`Loader`]: struct.Loader.html
    /// [`TextureArray`]: struct.TextureArray.html
    pub fn new(width: u16, height: u16) -> Loader {
        Loader {
            width,
            height,
            paths: Vec::new(),
        }
    }

    /// Queues an image to be added to the produced [`TextureArray`] and obtain
    /// a [`Key`] to its [`Index`].
    ///
    /// [`TextureArray`]: struct.TextureArray.html
    /// [`Key`]: struct.Key.html
    /// [`Index`]: struct.Index.html
    pub fn add<P: Into<PathBuf>>(&mut self, path: P) -> Key {
        self.paths.push(path.into());
        Key(self.paths.len() - 1)
    }

    /// Finishes the [`Loader`] definition and obtain a [`Task`] that produces
    /// a value from the loaded [`TextureArray`] and its [`Indices`].
    ///
    /// [`Loader`]: struct.Loader.html
    /// [`Task`]: ../../load/struct.Task.html
    /// [`TextureArray`]: struct.TextureArray.html
    /// [`Indices`]: struct.Indices.html
    pub fn finish<F, T>(self, on_completion: F) -> Task<T>
    where
        F: 'static + Fn(TextureArray, Indices) -> Result<T>,
    {
        let total_work = self.paths.len() as u32 + 1;

        Task::sequence(total_work, move |task| {
            let mut builder = Builder::new(self.width, self.height);
            let mut work_todo = VecDeque::from(self.paths.clone());
            let mut indices = Vec::new();

            while let Some(next) = work_todo.pop_front() {
                let index = builder.add(next)?;
                indices.push(index);

                task.notify_progress(1);
            }

            let result =
                on_completion(builder.build(task.gpu()), Indices(indices))?;

            task.notify_progress(1);

            Ok(result)
        })
    }
}

/// A key used to obtain an [`Index`] from [`Indices`] once a [`TextureArray`]
/// is loaded using a [`Loader`].
///
/// [`Key`]: struct.Key.html
/// [`Index`]: struct.Index.html
/// [`Indices`]: struct.Indices.html
/// [`TextureArray`]: struct.TextureArray.html
/// [`Loader`]: struct.Loader.html
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Key(usize);

/// A set of loaded indices obtained when using a [`Loader`].
///
/// [`Loader`]: struct.Loader.html
#[derive(Clone, PartialEq, Debug)]
pub struct Indices(Vec<Index>);

impl Indices {
    /// Get an [`Index`] for the given [`Key`].
    ///
    /// [`Key`]: struct.Key.html
    /// [`Index`]: struct.Index.html
    pub fn get(&self, key: Key) -> Result<Index> {
        self.0
            .get(key.0)
            .cloned()
            .ok_or(Error::TextureArray(super::Error::KeyNotFound(key.0)))
    }
}
