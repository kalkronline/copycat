use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use image_hasher::{HasherConfig, ImageHash};
use rayon::prelude::*;

use super::util;

#[derive(Debug)]
struct FileHash {
    file: PathBuf,
    hash: Option<ImageHash>,
}

pub struct Cheetah {
    done: Vec<FileHash>,
    todo: Vec<FileHash>,
}

impl Cheetah {
    pub fn new() -> Self {
        let done = Vec::new();
        let todo = Vec::new();
        Self { done, todo }
    }

    pub fn grab(&mut self, prey: &super::Options) -> anyhow::Result<()> {
        let mut files = HashSet::new();
        let dir = fs::read_dir(Path::new("./"))?;

        for file in dir {
            let file_str = util::pb_to_string(&file?.path());
            files.insert(file_str);
        }

        if prey.use_stow {
            self.get_stow(&mut files);
        }

        for k in files {
            let file = PathBuf::from(&k);
            if file == PathBuf::from("./.x3c") {
                continue;
            }
            self.todo.push(FileHash { file, hash: None });
        }

        Ok(())
    }

    fn get_stow(&mut self, files: &mut HashSet<String>) {
        let whole = fs::read_to_string(Path::new("./.x3c"));
        if whole.is_err() {
            return;
        }

        for line in whole.unwrap().split('\n').collect::<Vec<_>>() {
            let mut parts: Vec<_> = line.split(" // ").collect();

            let file = parts.pop();
            let hash = parts.pop();
            if file == None || hash == None {
                continue;
            }

            let hash = ImageHash::from_base64(hash.unwrap()).ok();
            let file = PathBuf::from(file.unwrap());
            let file_str = util::pb_to_string(&file);

            if files.remove(&file_str) {
                self.done.push(FileHash { file, hash })
            }
        }
    }

    pub fn digest(&mut self) {
        if self.todo.is_empty() {
            println!("copycat: nothing new to hash (used cache)");
            return;
        }

        println!("copycat: hashing images...");
        let progress = util::progress_bar(self.todo.len() as u64);

        let hasher = HasherConfig::new().to_hasher();
        self.todo.par_iter_mut().for_each(|fh| {
            let file = image::open(&fh.file);
            if let Ok(file) = file {
                let hash = hasher.hash_image(&file);
                progress.set_message(hash.to_base64());
                fh.hash = Some(hash);
            };
            progress.inc(1);
        });
        self.done.append(&mut self.todo);

        progress.finish_with_message("done");
    }

    pub fn stow(&mut self) -> anyhow::Result<()> {
        let mut outstr = String::new();
        for fh in &mut self.done {
            let file = fh.file.to_str().unwrap();
            let hash = match &fh.hash {
                None => "[unhashable]".to_owned(),
                Some(hash) => hash.to_base64(),
            };
            outstr.push_str(&format!("\n{} // {}", hash, file));
        }

        let cache_path = PathBuf::from("./.x3c");
        fs::write(cache_path, outstr)?;

        Ok(())
    }

    pub fn analyze(&mut self, prey: &super::Options) {
        let hashfiles = self
            .done
            .par_iter_mut()
            .flat_map(|fh| {
                if let Some(hash) = &fh.hash {
                    return Some((hash, &fh.file));
                }
                None
            })
            .collect::<Vec<_>>();

        let mut matches = hashfiles
            .clone()
            .iter()
            .zip(0..)
            .map(|((hash, _), i)| (*hash, Vec::new(), i))
            .collect::<Vec<_>>();

        matches.par_iter_mut().for_each(|(hash, matches, i)| {
            for (o_hash, o_file) in &hashfiles[*i + 1..] {
                if hash.dist(o_hash) < prey.distance + 1 {
                    matches.push(*o_file)
                }
            }
        });

        let matches = matches
            .iter()
            .flat_map(|(_, matches, _)| (!matches.is_empty()).then_some(matches))
            .collect::<Vec<_>>();

        for set in matches {
            println!("{:?}", set);
        }
    }
}
