use std::collections::HashSet;
use std::fs::ReadDir;
use std::path::PathBuf;

/// The type returned by the LynxIter iterator.
type Entry = std::path::PathBuf;

pub struct LynxIter {
    dir: ReadDir,
    pub exclude: HashSet<PathBuf>,
}

impl LynxIter {
    pub fn new(dir: ReadDir) -> Self {
        Self {
            dir,
            exclude: HashSet::new(),
        }
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
