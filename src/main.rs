use clap::Parser;
use std::{env, fs, path::Path};

mod kitty;

#[derive(Parser)]
#[command(version)]
/// Manage duplicate images
struct Args {
    #[arg(value_parser = check_path)]
    /// The target image directory
    path: String,
    #[arg(short, long)]
    /// Ignore any cache
    no_cache: bool,
    #[arg(short, long, default_value_t = 0)]
    /// The distance threshold (lower = more similar)
    distance: u32,
}

fn check_path(path: &str) -> Result<String, String> {
    let metadata = fs::metadata(path);

    if metadata.is_err() {
        if std::path::Path::new(path).exists() {
            return Err(format!("insufficient permissions for {}", path)); // cannot call metadata
        } else {
            return Err(format!("{} does not exist!", path)); // path doesn't exist
        }
    }

    let metadata = metadata.unwrap();

    if !metadata.is_dir() {
        return Err(format!("{} is not a directory!", path)); // path is not a directory
    }

    if fs::read_dir(path).is_err() {
        // is there a way to do this without fs::read_dir ?
        return Err(format!("insufficient permissions for {}", path)); // user does not have permissions
    }

    Ok(path.to_owned())
}

fn main() {
    let args = Args::parse();

    let dir = Path::new(&args.path);
    env::set_current_dir(dir).unwrap();

    let options = kitty::Options::new()
        .uses_stow(!args.no_cache)
        .distance(args.distance);

    kitty::run(options).unwrap();
}
