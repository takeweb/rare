use std::{env, path};

pub fn run(target: &path::PathBuf, level: isize, exclusion: &str) {
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
            run(&path, level + 1, &exclusion);
            continue;
        }
        println!("|-- <{}>", fname);
    }
}

pub fn get_current_dir(target_path: &str) -> String {
    if target_path == "." {
        // 現在のディレクトリを返す
        let pwd = env::current_dir().unwrap();
        return pwd.to_str().unwrap().to_string();
    }
    target_path.to_string()
}
