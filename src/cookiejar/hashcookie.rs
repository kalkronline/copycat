use std::path::{Path, PathBuf};

use image_hasher::{Hasher, ImageHash};

pub struct HashGramma;

pub struct HashCookie {
    pub hash: Option<ImageHash>,
    pub path: PathBuf,
}

impl super::Gramma for HashGramma {
    type Cookie = HashCookie;

    fn cookie(&self, kvstring: &str) -> Option<Self::Cookie> {
        let mut parts = kvstring.split(" // ");

        if let (Some(f), Some(s)) = (parts.next(), parts.next()) {
            let path = Path::new(s).to_owned();
            let hash = if f == "[unhashable]" {
                None
            } else {
                Some(ImageHash::from_base64(f).ok()?)
            };

            Some(HashCookie { hash, path })
        } else {
            None
        }
    }

    fn devour(&self, cookie: Self::Cookie) -> Option<String> {
        let path = cookie.path.to_str()?;
        let hash = match cookie.hash {
            Some(hash) => hash.to_base64(),
            None => "[unhashable]".into(),
        };
        Some(hash + " // " + path)
    }
}

impl HashCookie {
    pub fn new(path: PathBuf) -> Self {
        Self { hash: None, path }
    }

    pub fn hash(&mut self, hasher: &Hasher) -> anyhow::Result<()> {
        let image = image::open(&self.path)?;
        self.hash = Some(hasher.hash_image(&image));
        Ok(())
    }
}
