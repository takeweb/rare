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

    // コマンドをインスタンス化
    let mut cmd = rtree::RtreeCmd::new(args.exclusions);

    // ルートディレクトリを出力
    cmd.print_current_dir(&args.target_dir);

    // PathBufに変換
    let target_path = path::PathBuf::from(&args.target_dir);
    cmd.run(&target_path, 0);

    // ディレクトリ、ファイルのカウントを出力
    cmd.print_cnt();
}
