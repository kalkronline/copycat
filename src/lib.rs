mod cheetah;
mod cookiejar;
mod lynx;

use std::path::Path;

use cheetah::Cheetah;

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Default)]
pub struct Options {
    use_cache: bool,
    distance: u32,
}

impl Options {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn use_cache(mut self, uses: bool) -> Self {
        self.use_cache = uses;
        self
    }

    pub fn distance(mut self, dist: u32) -> Self {
        self.distance = dist;
        self
    }
}

pub fn hash(opts: Options) -> anyhow::Result<()> {
    let style =
        ProgressStyle::with_template("{elapsed_precise} {bar:40.white/black} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-");

    let mut cheetah = Cheetah::new(Path::new("./").into());

    if opts.use_cache {
        cheetah.exclude_cache();
    }

    let progress = ProgressBar::new(cheetah.todo_len() as u64);
    progress.set_style(style);

    cheetah.hash(|hash| {
        if let Some(hash) = hash {
            progress.set_message(hash.to_base64())
        }
        progress.inc(1);
    });

    progress.finish_with_message("done");

    Ok(())
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    hash(opts)?;

    Ok(())
}
