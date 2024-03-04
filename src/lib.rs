use std::{
    env,
    fs::{self, DirEntry},
    path,
};

#[derive(Clone)]
struct LastInfo {
    last_name: String,
    last_level: isize,
    last_flg: bool,
}

/// ツリーコマンド
#[derive(Clone)]
pub struct RtreeCmd {
    d_cnt: isize,
    f_cnt: isize,
    exclusions: Vec<String>,
    current_flg: bool,
    last_infos: Vec<LastInfo>,
}
impl RtreeCmd {
    /// ツリーコマンドの初期化
    pub fn new(exclusions: Vec<String>, current_flg: bool) -> Self {
        let last_infos = vec![];
        RtreeCmd {
            d_cnt: 0,
            f_cnt: 0,
            exclusions,
            current_flg,
            last_infos,
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
        let last: &&DirEntry = &files.last().unwrap();
        let last_path = last.path();
        let last_fname = last_path.file_name().unwrap().to_string_lossy();
        if last_path.is_dir() {
            let last = LastInfo {
                last_name: last_fname.to_string(),
                last_level: level + 1,
                last_flg: false,
            };
            self.last_infos.push(last);
        }

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
            for current_level in 1..=level {
                let target_fname = target_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                // println!(
                //     "current_level: {}, target_fname: {}",
                //     current_level, target_fname
                // );

                // 現在処理中のディレクトリが、その階層で最後かを判定
                let result_last = &self
                    .last_infos
                    .iter()
                    .position(|x| x.last_name == target_fname && x.last_level == current_level);
                let last_flg = match result_last {
                    Some(i) => {
                        self.last_infos.get_mut(*i).unwrap().last_flg = true;
                        true
                    }
                    None => false,
                };

                // 現在処理中のディレクトリ又はファイルが、最終階層配下かを判定
                let result_last_child = &self.last_infos.iter().position(|x| {
                    (x.last_name == target_fname
                        && x.last_level == current_level
                        && x.last_flg == true)
                        || (x.last_level == current_level && x.last_flg == true)
                        || (x.last_level - 1 >= current_level && x.last_flg == true)
                });
                let last_flg = match result_last_child {
                    Some(i) => {
                        self.last_infos.get_mut(*i).unwrap().last_flg = true;
                        true
                    }
                    None => false,
                };

                if last_flg {
                    print!("     ")
                } else {
                    print!("│    ")
                }
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
            }
        }
    }

    /// ルートディレクトリを出力
    ///
    /// * `target_dir` - 対象ディレクトリ
    pub fn print_current_dir(&mut self, target_dir: &str) {
        self.d_cnt += 1;
        let result = if target_dir == "." && self.current_flg {
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

    pub fn print_last(&self) {
        for item in self.last_infos.iter() {
            println!(
                "last_name: {} last_level: {} last_flg: {}",
                item.last_name, item.last_level, item.last_flg
            );
        }
    }
}
