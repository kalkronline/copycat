use image_hasher::{HasherConfig, ImageHash};
use rayon::prelude::*;

use crate::cookiejar::{CookieJar, HashCookie, HashGramma};
use crate::lynx::Lynx;

pub struct Cheetah<'a> {
    cookiejar: &'a mut CookieJar<HashGramma>,
    links: Lynx<HashCookie>,
}

/// Hash files fastly
impl<'a> Cheetah<'a> {
    pub fn new(cookiejar: &'a mut CookieJar<HashGramma>) -> anyhow::Result<Self> {
        let mut links = Lynx::new("./")?;
        links.exclude("./.x3c");
        links.exclude_many(cookiejar.jar().iter().map(|cookie| &cookie.path));

        Ok(Self { links, cookiejar })
    }

    pub fn len(&mut self) -> usize {
        self.links.compute(HashCookie::new);
        self.links.size()
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
