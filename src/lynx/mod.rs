use std::fs;
use std::path::{Path, PathBuf};

use self::Lynx::{Done, Res, Todo};
use iter::LynxIter;

mod iter;

/// Represents a future directory gathering effort.
///
/// Many `Lynx` methods panic. Ensure that you are using them correctly.
pub enum Lynx<T> {
    Todo(LynxIter),
    Res(Vec<T>),
    Done,
}

impl<T> Lynx<T> {
    /// Returns a new `Lynx` as long as the provided path is valid.
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Todo(LynxIter::new(fs::read_dir(path)?)))
    }

    /// Modifies the `Todo` iterator by adding item to its exclude list.
    ///
    /// Panics when `self` is not `Todo`.
    pub fn exclude<P: AsRef<Path>>(&mut self, path: P) {
        match self {
            Todo(iter) => {
                iter.exclude.insert(path.as_ref().to_owned());
            }
            _ => panic!(),
        }
    }

    /// Modifies the `Todo` iterator by adding items to its exclude list.
    ///
    /// Panics when `self` is not `Todo`.
    pub fn exclude_many<P: AsRef<Path>, I: IntoIterator<Item = P>>(&mut self, paths: I) {
        match self {
            Todo(iter) => {
                for path in paths {
                    iter.exclude.insert(path.as_ref().to_owned());
                }
            }
            _ => panic!(),
        }
    }

    /// Transforms the `Todo` iterator into a `Vec<T>` by some function.
    pub fn compute<F: Fn(PathBuf) -> T>(&mut self, func: F) {
        let iter = match self {
            Todo(iter) => iter,
            _ => return,
        };

        let res = iter.map(func).collect();
        let _ = std::mem::replace(self, Res(res));
    }

    /// Gets the size of the result vec.
    ///
    /// Panics when `self` is not `Res`.
    pub fn size(&self) -> usize {
        match self {
            Res(res) => res.len(),
            _ => panic!(),
        }
    }

    /// Converts `self` into `Done`, and returns results.
    ///
    /// Panics when `self` is not `Res`.
    pub fn consume(&mut self) -> Vec<T> {
        match std::mem::replace(self, Done) {
            Res(res) => res,
            _ => panic!(),
        }
    }
}
