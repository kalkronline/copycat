// how to compare hashes
// steps

// get all hashes
// (multi)
// for each hash with id n, copy a slice of n+1 to end
//     on first match create a hashset
//     on any match, push id m of the match into hashset
//     return Some((n, hashset)) if hashset else None
// collect into hm
// (single)
//

// 1 (5)
// 2 (3, 6)
// 3 (6)
// 5 (7)

// start at top

use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::cookiejar::{CookieJar, HashGramma};

pub struct Kitty;

impl Kitty {
    pub fn new(cookies: &CookieJar<HashGramma>) -> Self {
        // first pass, needs a bigger set to be able to tell if this helps at all

        for _ in 0..10 {
            let dist = 0;

            let hashes: Vec<_> = cookies
                .jar()
                .iter()
                .zip(0..)
                .flat_map(|(cookie, id)| cookie.hash.as_ref().map(|hash| (hash, id)))
                .collect();

            let mut matches_map: HashMap<_, _> = hashes
                .par_iter()
                .flat_map(|(hash, id)| {
                    let mut matches: Option<HashSet<usize>> = None;

                    for o_hash in &hashes[id + 1..] {
                        if o_hash.0.dist(hash) < (dist + 1) {
                            if let Some(set) = &mut matches {
                                set.insert(o_hash.1);
                            } else {
                                let mut set = HashSet::new();
                                set.insert(o_hash.1);
                                matches = Some(set);
                            }
                        };
                    }

                    matches.map(|set| (id, set))
                })
                .collect();

            let mut keys: Vec<_> = matches_map.keys().copied().collect();
            // keys.sort(); // may increase pref ???

            for key in keys {
                if let Some(mut set) = matches_map.remove(key) {
                    let mut others = Vec::new();

                    for other_key in set.iter() {
                        if let Some(other_set) = matches_map.remove(other_key) {
                            others.push(other_set);
                        }
                    }

                    others.into_iter().for_each(|other| set.extend(other));

                    matches_map.insert(key, set);
                }
            }

            println!("{:?}", matches_map);
        }
        Self
    }
}
