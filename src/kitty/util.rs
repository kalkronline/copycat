use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;

pub use log::progress_bar;

mod log {
    use indicatif::{ProgressBar, ProgressStyle};

    pub fn progress_bar(len: u64) -> ProgressBar {
        let style = ProgressStyle::with_template(
            "{elapsed_precise} {bar:40.white/black} {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("##-");

        let progress = ProgressBar::new(len);
        progress.set_style(style);

        progress
    }
}

pub fn pb_to_string(pb: &Path) -> String {
    let str: &str = &pb.to_string_lossy();
    str.to_owned()
}

pub struct RichardSet<T: Eq + Hash> {
    // terrible!!!!!!!!
    vec: Vec<Vec<Rc<T>>>,
    map: HashMap<Rc<T>, usize>,
}

impl<T: Eq + Hash> RichardSet<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, one: T, two: T) {
        let oneidx = self.map.get(&one).copied();
        let twoidx = self.map.get(&two).copied();

        match (oneidx, twoidx) {
            (None, None) => self.new_set(one, two),
            (None, Some(idx)) => self.push_set(idx, one),
            (Some(idx), None) => self.push_set(idx, two),
            _ => (),
        }
    }

    fn new_set(&mut self, one: T, two: T) {
        self.vec.push(Vec::new());

        let idx = self.vec.len() - 1;
        let one_p = Rc::new(one);
        let two_p = Rc::new(two);

        self.map.insert(one_p.clone(), idx);
        self.map.insert(two_p.clone(), idx);
        self.vec[idx].push(one_p);
        self.vec[idx].push(two_p);
    }

    fn push_set(&mut self, to: usize, item: T) {
        let item_p = Rc::new(item);

        self.map.insert(item_p.clone(), to);
        self.vec.get_mut(to).unwrap().push(item_p);
    }

    pub fn collect(self) -> Vec<Vec<T>> {
        drop(self.map);

        // bad!!!!!!!!
        self.vec
            .into_iter()
            .map(|vec| {
                vec.into_iter()
                    .map(|rc| Rc::try_unwrap(rc).ok().unwrap())
                    .collect()
            })
            .collect()
    }
}
