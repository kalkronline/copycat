use cheetah::Cheetah;

mod cheetah;
pub mod util;

pub struct Options {
    use_stow: bool,
    distance: u32,
}

impl Options {
    pub fn new() -> Self {
        Self {
            use_stow: true,
            distance: 0,
        }
    }

    pub fn uses_stow(mut self, uses: bool) -> Self {
        self.use_stow = uses;
        self
    }

    pub fn distance(mut self, dist: u32) -> Self {
        self.distance = dist;
        self
    }
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    let mut gatherer = Cheetah::new();
    gatherer.grab(&opts)?;
    gatherer.digest();
    gatherer.stow()?;
    gatherer.analyze(&opts);

    Ok(())
}
