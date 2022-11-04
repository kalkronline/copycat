use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Manage duplicate images
struct Args {
    /// The target image directory
    path: String,
    #[arg(short, long)]
    /// Ignore caches, will still write
    nocache: bool,
    #[arg(short, long, default_value_t = 0)]
    /// The distance threshold (lower = more similar)
    distance: u32,
}


fn main() {
    let args = Args::parse();

    let threads = std::thread::available_parallelism().unwrap().get();
    rayon::ThreadPoolBuilder::new().num_threads(threads / 4 * 3).build_global().unwrap();

    let options = copycat::Options::new()
        .use_cache(!args.nocache)
        .distance(args.distance);

    copycat::run(&args.path, options).unwrap();
}
