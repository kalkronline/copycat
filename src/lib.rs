use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use cheetah::Cheetah;
use cookiejar::{CookieJar, HashGramma};
use kitty::Kitty;
pub use options::Options;

mod cheetah;
mod cookiejar;
mod kitty;
mod lynx;
mod options;

fn hash(cookies: &mut CookieJar<HashGramma>) -> anyhow::Result<()> {
    let mut cheetah = Cheetah::new(cookies)?;

    if cheetah.len() == 0 {
        println!("nothing to hash!");
        return Ok(());
    }

    let style = "{elapsed_precise} {bar:40.white/black} {pos}/{len} {msg}";
    let style = ProgressStyle::with_template(style)?.progress_chars("##-");
    let progress = ProgressBar::new(cheetah.len() as u64).with_style(style);

    cheetah.hash(|hash| {
        if let Some(hash) = hash {
            progress.set_message(hash.to_base64())
        }
        progress.inc(1);
    });

    progress.finish_with_message("done");

    Ok(())
}

fn matches(cookies: &CookieJar<HashGramma>) {
    Kitty::new(cookies);
}

pub fn run<P: AsRef<Path>>(path: P, opts: Options) -> anyhow::Result<()> {
    std::env::set_current_dir(path)?;

    let mut cookiejar = CookieJar::new("./.x3c", HashGramma)?;

    if opts.use_cache {
        cookiejar.add_old()?;
    }

    hash(&mut cookiejar)?;
    matches(&cookiejar);

    cookiejar.save()?;

    Ok(())
}
