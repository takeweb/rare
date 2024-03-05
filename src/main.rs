use clap::Parser;
use std::path;

/// Rust cat CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Target directory
    #[arg(value_name = "TARGET DIR", default_value = "./tests")]
    target_dir: String,

    /// Exclusion Keywords
    #[arg(
        short = 'e',
        long = "exclusions",
        required = false,
        value_delimiter(','),
        default_value = ".DS_Store,.git,.gitignore,.vscode"
    )]
    exclusions: Vec<String>,

    /// Display current directory
    #[arg(
        short = 'c',
        long = "current_dir",
        required = false,
        default_value_t = false
    )]
    current_flg: bool,
}

fn main() {
    // コマンドライン引数を解析
    let args = Args::parse();

    // コマンドをインスタンス化
    let mut cmd = rtree::RtreeCmd::new(args.exclusions, args.current_flg);

    // ルートディレクトリを出力
    cmd.print_current_dir(&args.target_dir);

    // コマンド実行
    cmd.run(&path::PathBuf::from(&args.target_dir), 0);

    // ディレクトリ、ファイルのカウントを出力
    cmd.print_cnt();

    cmd.print_last();
}
