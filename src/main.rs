use std::{env, fs, path::Path};

use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Manage duplicate images
struct Args {
    /// The target image directory
    path: String,
    #[arg(short, long)]
    /// Ignore any cache
    no_cache: bool,
    #[arg(short, long, default_value_t = 0)]
    /// The distance threshold (lower = more similar)
    distance: u32,
}

fn main() {
    let args = Args::parse();

    let options = copycat::Options::new()
        .use_cache(!args.no_cache)
        .distance(args.distance);

    copycat::run(&args.path, options).unwrap();
}
