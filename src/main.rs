use std::{env, path};

fn main() {
    // コマンドライン引数を確認
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    let mut exclusion = "";

    // カレントディレクトリを指定
    if args.len() >= 2 {
        target_dir = &args[1];
    }
    if args.len() >= 3 {
        exclusion = &args[2];
    }

    // PathBufに変換
    let target = path::PathBuf::from(target_dir);
    println!("{}", get_current_dir(target_dir));

    tree(&target, 0, &exclusion);
}

fn tree(target: &path::PathBuf, level: isize, exclusion: &str) {
    // ファイル一覧を取得
    let files = target.read_dir().expect("存在しないパス");

    for ent in files {
        let path = ent.unwrap().path();

        for _ in 1..=level {
            print!("|  ");
        }

        let fname = path.file_name().unwrap().to_string_lossy();

        if exclusion != "" && None != fname.find(exclusion) {
            continue;
        }

        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level + 1, &exclusion);
            continue;
        }
        println!("|-- <{}>", fname);
    }
}

fn get_current_dir(target_path: &str) -> String {
    if target_path == "." {
        // 現在のディレクトリを返す
        let pwd = env::current_dir().unwrap();
        return pwd.to_str().unwrap().to_string();
    }
    target_path.to_string()
}
