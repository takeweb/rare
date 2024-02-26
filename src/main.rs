use clap::Parser;
use std::path;

/// Rust cat CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target directory
    #[arg(value_name = "TARGET DIR", default_value = ".")]
    target_dir: String,

    /// Exclusion Keywords
    #[arg(
        short = 'e',
        long = "exclusions",
        required = false,
        value_delimiter(',')
    )]
    exclusions: Vec<String>,
}

fn main() {
    // コマンドライン引数を解析
    let args = Args::parse();

    // PathBufに変換
    let target = path::PathBuf::from(&args.target_dir);
    println!("{}", rtree::get_current_dir(&args.target_dir));

    rtree::run(&target, 0, &args.exclusions);
}
