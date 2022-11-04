use std::collections::HashSet;
use std::ops::Index;

use rayon::prelude::*;

use crate::cookiejar::{CookieJar, HashCookie, HashGramma};

pub struct Kitty<'a> {
    pub matches: Vec<Vec<&'a HashCookie>>,
}

impl<'a> Kitty<'a> {
    pub fn new(cookies: &'a CookieJar<HashGramma>, dist: u32) -> Self {
        let cookies = cookies.jar();

        // prepare
        let hashes: Vec<_> = cookies
            .iter()
            .zip(0..)
            .flat_map(|(cookie, id)| cookie.hash.as_ref().map(|hash| (hash, id)))
            .collect();

        // match
        let mut match_sets: Vec<_> = hashes
            .par_iter()
            .flat_map(|(hash, id)| {
                let mut matches: Option<HashSet<usize>> = None;

                for o_hash in &hashes[id + 1..] {
                    if o_hash.0.dist(hash) < (dist + 1) {
                        if let Some(set) = &mut matches {
                            set.insert(o_hash.1);
                        } else {
                            let mut set = HashSet::new();
                            set.insert(*id);
                            set.insert(o_hash.1);
                            matches = Some(set);
                        }
                    };
                }

                matches
            })
            .collect();

        let mut matches = Vec::new();

        while let Some(mut set) = match_sets.pop() {
            for i in (0..match_sets.len()).rev() {
                if !set.is_disjoint(&match_sets[i]) {
                    set.extend(match_sets.swap_remove(i));
                }
            }
            let refs: Vec<_> = set.into_iter().map(|idx| cookies.index(idx)).collect();
            matches.push(refs);
        }

        Self { matches }
    }

    pub fn print(&self) {
        for group in &self.matches {
            println!();
            for cookie in group {
                let fp = cookie.path.canonicalize().unwrap();
                let fp = fp.display();
                println!("{} {}", cookie.hash.as_ref().unwrap().to_base64(), fp);
            }
        }
    }
}
