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
        let mut links = Lynx::new(&path).unwrap();
        path.push(".x3c");
        links.exclude(&path);
        let cookiejar = CookieJar::new(path, HashGramma).unwrap();

        Self { links, cookiejar }
    }

    pub fn len(&mut self) -> usize {
        self.links.compute(HashCookie::new);
        self.links.size()
    }

    pub fn use_cache(&mut self, bool: bool) {
        // cookiejar probably should be an outside thing
        if bool {
            self.links.exclude_many(self.cookiejar.taste());
        } else {
            self.cookiejar.clear();
            self.cookiejar.no_save();
        }
    }

    pub fn hash<F>(&mut self, progress: F)
    where
        F: Fn(&Option<ImageHash>) + Sync,
    {
        self.links.compute(HashCookie::new);
        let mut cookies = self.links.consume();

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
