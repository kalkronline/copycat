use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

/// The type returned by the LynxIter iterator.
type Entry = std::path::PathBuf;

#[derive(Default)]
pub struct LynxOptions {
    path: PathBuf,
    exclude: Vec<PathBuf>,
}

pub enum Lynx<T> {
    Options(LynxOptions),
    Results(Vec<T>),
}

impl<T> Lynx<T> {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let options = LynxOptions {
            path: path.as_ref().to_owned(),
            exclude: vec![],
        };
        Self::Options(options)
    }

    pub fn compute<F: Fn(PathBuf) -> T>(&mut self, func: F) -> Option<Self> {
        if let Self::Options(opts) = self {
            let opts = std::mem::take(opts);
            let dir_iter = fs::read_dir(opts.path).unwrap();
            let links = LynxIter::new(dir_iter, opts.exclude);

            let stuff = links.map(func).collect::<Vec<_>>();
            Some(Self::Results(stuff))
        } else {
            None
        }
    }

    pub fn size(&mut self) -> Option<usize> {
        if let Self::Results(res) = self {
            Some(res.len())
        } else {
            None
        }
    }

    pub fn results(self) -> Option<Vec<T>> {
        if let Self::Results(res) = self {
            Some(res)
        } else {
            None
        }
    }

    pub fn exclude<P: AsRef<Path>>(&mut self, path: P) -> Option<()> {
        if let Self::Options(opts) = self {
            opts.exclude.push(path.as_ref().to_owned());
            Some(())
        } else {
            None
        }
    }

    pub fn exclude_many<P: AsRef<Path>, I: Iterator<Item = P>>(&mut self, paths: I) -> Option<()> {
        if let Self::Options(opts) = self {
            for path in paths {
                opts.exclude.push(path.as_ref().to_owned());
            }
            Some(())
        } else {
            None
        }
    }
}

pub struct LynxIter {
    dir: ReadDir,
    exclude: Vec<PathBuf>,
}

impl LynxIter {
    fn new(dir: ReadDir, exclude: Vec<PathBuf>) -> Self {
        Self { dir, exclude }
    }

    fn seek_till_ok(&mut self) -> Option<Entry> {
        loop {
            if let Some(res) = self.dir.next() {
                if let Ok(item) = res {
                    return Some(item.path());
                }
            } else {
                return None;
            }
        }
    }

    fn seek_till_include(&mut self) -> Option<Entry> {
        loop {
            if let Some(res) = self.seek_till_ok() {
                if !self.exclude.contains(&res) {
                    return Some(res);
                }
            } else {
                return None;
            }
        }
    }
}

impl Iterator for LynxIter {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.seek_till_include()
    }
}
