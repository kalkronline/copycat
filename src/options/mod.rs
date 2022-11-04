#[derive(Default)]
pub struct Options {
    pub(super) use_cache: bool,
    pub(super) distance: u32,
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
