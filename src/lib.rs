use std::{
    env,
    fs::{self, DirEntry},
    path,
};

/// ツリーコマンド
#[derive(Clone)]
pub struct RtreeCmd {
    d_cnt: isize,
    f_cnt: isize,
    exclusions: Vec<String>,
}
impl RtreeCmd {
    /// ツリーコマンドの初期化
    pub fn new(exclusions: Vec<String>) -> Self {
        RtreeCmd {
            d_cnt: 0,
            f_cnt: 0,
            exclusions,
        }
    }

    /// ディレクトリ、ファイル表示実行
    ///
    /// * `target_path` - 対象ディレクトリ
    /// * `level` - 階層レベル
    pub fn run(&mut self, target_path: &path::PathBuf, level: isize) {
        // ファイル一覧を取得
        let mut files: Vec<DirEntry> = fs::read_dir(target_path)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        // ファイル一覧をソート
        files.sort_by_key(|dir| dir.path());

        // 最後の要素を取り出し
        let last = &files.last().unwrap();
        let last_path = last.path();
        let last_fname = last_path.file_name().unwrap().to_string_lossy();

        // ファイル一覧分処理を実行
        for ent in files {
            // パスからファイル名を取り出し
            let path = ent.path();
            let fname = path.file_name().unwrap().to_string_lossy();

            // 除外ファイル判定
            let result = &self.exclusions.iter().position(|x| x == &fname.to_string());
            let _ = match result {
                Some(_) => continue,
                None => 0,
            };

            // 階層出力
            for _ in 1..=level {
                print!("│    ");
            }

            // 対象のパスがファイルの場合
            if path.is_file() {
                self.f_cnt += 1;

                if fname.eq_ignore_ascii_case(&last_fname) {
                    println!("└── {}", fname);
                } else {
                    println!("├── {}", fname);
                }
            }

            // 対象のパスがディレクトリの場合
            if path.is_dir() {
                if fname.eq_ignore_ascii_case(&last_fname) {
                    println!("└── [{}]", fname);
                } else {
                    println!("├── [{}]", fname);
                }
                self.d_cnt += 1;

                // 再帰呼び出し
                self.run(&path, level + 1);
                continue;
            }
        }
    }

    /// ルートディレクトリを出力
    ///
    /// * `target_dir` - 対象ディレクトリ
    pub fn print_current_dir(&mut self, target_dir: &str) {
        self.d_cnt += 1;
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
