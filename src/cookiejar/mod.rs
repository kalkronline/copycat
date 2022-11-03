use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::Path;

mod hashcookie;
pub use hashcookie::{HashCookie, HashGramma};

/// A gramma creates cookies by providing methods which
/// convert a pair of strings to and from cookies.
pub trait Gramma
where
    Self::Unique: Hash + Eq,
{
    type Cookie;
    type Unique;

    /// creates a cookie from a string
    fn cookie(&self, string: &str) -> Option<Self::Cookie>;

    /// gets some part of a cookie
    fn taste(&self, cookie: &Self::Cookie) -> Self::Unique;

    /// turns a cookie into a string
    fn devour(&self, cookie: Self::Cookie) -> Option<String>;
}

pub struct CookieJar<G: Gramma> {
    file: File,
    saves: bool,
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

        let mut text = String::new();
        BufReader::new(&file).read_to_string(&mut text).unwrap();

        let mut jar = Vec::new();
        for line in text.split('\n') {
            if let Some(cookie) = gramma.cookie(line) {
                jar.push(cookie);
            }
        }

        let saves = true;

        Ok(Self {
            file,
            gramma,
            jar,
            saves,
        })
    }

    pub fn add(&mut self, cookie: G::Cookie) {
        self.jar.push(cookie);
    }

    pub fn no_save(&mut self) {
        self.saves = false;
    }

    pub fn clear(&mut self) {
        self.jar = Vec::new();
    }

    pub fn taste(&self) -> Box<dyn Iterator<Item = G::Unique> + '_> {
        Box::new(self.jar.iter().map(|cookie| self.gramma.taste(cookie)))
    }
}

impl<G: Gramma> Drop for CookieJar<G> {
    fn drop(&mut self) {
        if !self.saves {
            return;
        }

        let mut leftovers = Vec::new();

        for cookie in std::mem::take(&mut self.jar) {
            if let Some(crumbs) = self.gramma.devour(cookie) {
                leftovers.push(crumbs)
            }
        }

        let leftovers = leftovers.join("\n");
        let buf = leftovers.as_bytes();
        let msg = "SOMETHINGS GONE TERRIBLY WRONG!! \
                   an error occured on write. check for corrupt data.";

        self.file.rewind().expect(msg);
        self.file.write_all(buf).expect(msg);
        self.file.set_len(buf.len() as u64).expect(msg);
    }
}
