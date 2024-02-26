use std::{env, fs, path};

/// ツリーコマンド
#[derive(Clone, Copy)]
pub struct RtreeCmd {
    d_cnt: isize,
    f_cnt: isize,
}
impl RtreeCmd {
    /// ツリーコマンドの初期化
    pub fn new() -> Self {
        RtreeCmd { d_cnt: 0, f_cnt: 0 }
    }

    /// ディレクトリ、ファイル表示実行
    ///
    /// * `target_path` - 対象ディレクトリ
    /// * `level` - 階層レベル
    /// * `exclusions` - 除外キーワード
    pub fn run(&mut self, target_path: &path::PathBuf, level: isize, exclusions: &Vec<String>) {
        // ファイル一覧を取得
        let mut files: Vec<_> = fs::read_dir(target_path)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        files.sort_by_key(|dir| dir.path());

        for ent in files {
            let path = ent.path();

            for _ in 1..=level {
                print!("│   ");
            }

            let fname = path.file_name().unwrap().to_string_lossy();

            let mut ex_flg = false;
            for exclusion in exclusions {
                if exclusion != "" && None != fname.find(exclusion) {
                    ex_flg = true;
                    continue;
                }
            }
            if ex_flg {
                continue;
            }

            if path.is_file() {
                self.f_cnt += 1;
            }

            if path.is_dir() {
                println!("├── {}", fname);
                self.d_cnt += 1;
                self.run(&path, level + 1, &exclusions);
                continue;
            }
            println!("├── {}", fname);
        }
    }

    /// ルートディレクトリを出力
    ///
    /// * `target_dir` - 対象ディレクトリ
    pub fn print_current_dir(&self, target_dir: &str) {
        let result = if target_dir == "." {
            // 現在のディレクトリを返す
            let pwd = env::current_dir().unwrap();
            pwd.to_str().unwrap().to_string()
        } else {
            target_dir.to_string()
        };

        // ルートディレクトリを出力
        println!("{}", result);
    }

    /// ディレクトリ、ファイルのカウントを出力
    pub fn print_cnt(&self) {
        println!("");
        println!("{} directories, {} files", self.d_cnt, self.f_cnt);
    }
}
