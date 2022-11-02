use image_hasher::{HasherConfig, ImageHash};
use rayon::prelude::*;

use crate::cookiejar::{CookieJar, HashCookie, HashGramma};
use crate::lynx::Lynx;

use std::path::PathBuf;

pub struct Cheetah {
    cookiejar: CookieJar<HashGramma>,
    links: Lynx<HashCookie>,
}

impl Cheetah {
    pub fn new(mut path: PathBuf) -> Self {
        let mut links = Lynx::new(&path);
        path.push(".x3c");
        links.exclude(&path);
        let cookiejar = CookieJar::new(path, HashGramma).unwrap();

        Self { links, cookiejar }
    }

    pub fn todo_len(&mut self) -> usize {
        self.links = self.links.compute(HashCookie::new).unwrap();
        self.links.size().unwrap()
    }

    pub fn exclude_cache(&mut self) {
        self.links.exclude_many(self.cookiejar.taste_test());
    }

    pub fn hash<F>(mut self, progress: F)
    where
        F: Fn(&Option<ImageHash>) + Sync,
    {
        self.links = self.links.compute(HashCookie::new).unwrap_or(self.links);
        let mut cookies = self.links.results().unwrap();

        let hasher = HasherConfig::new().to_hasher();

        cookies.par_iter_mut().for_each(|cookie| {
            let _ = cookie.hash(&hasher);
            progress(&cookie.hash);
        });

        for cookie in cookies {
            self.cookiejar.add(cookie);
        }
    }
}
