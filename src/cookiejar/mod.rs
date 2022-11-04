use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::Path;

pub use hashcookie::{HashCookie, HashGramma};

mod hashcookie;

/// A gramma creates cookies by providing methods which
/// convert a pair of strings to and from cookies.
pub trait Gramma {
    type Cookie;

    /// creates a cookie from a string
    fn cookie(&self, string: &str) -> Option<Self::Cookie>;

    /// turns a cookie into a string
    fn devour(&self, cookie: Self::Cookie) -> Option<String>;
}

pub struct CookieJar<G: Gramma> {
    file: File,
    gramma: G,
    jar: Vec<G::Cookie>,
}

impl<G: Gramma> CookieJar<G> {
    pub fn new<P: AsRef<Path>>(at: P, gramma: G) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(at)?;

        let jar = Vec::new();

        Ok(Self { file, gramma, jar })
    }

    pub fn add_old(&mut self) -> anyhow::Result<()> {
        let mut text = String::new();
        BufReader::new(&self.file)
            .read_to_string(&mut text)
            .unwrap();

        for line in text.split('\n') {
            if let Some(cookie) = self.gramma.cookie(line) {
                self.jar.push(cookie);
            }
        }

        Ok(())
    }

    pub fn add(&mut self, cookie: G::Cookie) {
        self.jar.push(cookie);
    }

    pub fn jar(&self) -> &Vec<G::Cookie> {
        &self.jar
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let mut leftovers = Vec::new();

        for cookie in std::mem::take(&mut self.jar) {
            if let Some(crumbs) = self.gramma.devour(cookie) {
                leftovers.push(crumbs)
            }
        }

        let leftovers = leftovers.join("\n");
        let buf = leftovers.as_bytes();

        self.file.rewind()?;
        self.file.write_all(buf)?;
        self.file.set_len(buf.len() as u64)?;

        Ok(())
    }
}
