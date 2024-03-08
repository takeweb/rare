use std::{
    env,
    fs::{self, DirEntry},
    path,
};

use regex::Regex;

#[derive(Clone)]
struct LastInfo {
    level: isize,
    name: String,
    flg: bool,
}

/// ツリーコマンド
#[derive(Clone)]
pub struct RtreeCmd {
    d_cnt: isize,              // ディレクトリ数カウント
    f_cnt: isize,              // ファイル数カウント
    exclusions: Vec<String>,   // 除外ファイル名
    current_flg: bool,         // カレントディレクトリを表示する/しない
    last_infos: Vec<LastInfo>, // 各階層の最終ディレクトリ情報
}
impl RtreeCmd {
    /// ツリーコマンドの初期化
    pub fn new(exclusions: Vec<String>, current_flg: bool) -> Self {
        RtreeCmd {
            d_cnt: 0,
            f_cnt: 0,
            exclusions,
            current_flg,
            last_infos: vec![],
        }
    }

    /// ディレクトリ、ファイル表示実行
    ///
    /// * `target_path` - 対象ディレクトリ
    /// * `level` - 階層レベル
    pub fn run(&mut self, target_path: &path::PathBuf, level: isize) {
        // 対象ディレクトリ名取得
        let target_dname = target_path.to_string_lossy().to_string();

        // 配下のファイル一覧を取得
        let mut files: Vec<DirEntry> = fs::read_dir(target_path)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        // ファイル一覧をソート
        files.sort_by_key(|dir| dir.path());

        // 最後の要素を取り出し
        let mut last_fname = String::from("");
        if files.len() > 0 {
            let last: &&DirEntry = &files.last().unwrap();
            let last_path = last.path();
            last_fname = last_path.file_name().unwrap().to_string_lossy().to_string();

            let result_lower = &self
                .last_infos
                .iter()
                .position(|last| last.level == level + 1);
            match result_lower {
                // 既に同一階層の要素があったら、削除
                Some(i) => {
                    self.last_infos.remove(*i);
                    Some(true)
                }
                None => None,
            };

            if last_path.is_dir() {
                let last = LastInfo {
                    level: level + 1,
                    name: last_path.to_string_lossy().to_string(),
                    flg: false,
                };
                self.last_infos.push(last);
            }
        }

        // 現在処理中のディレクトリが、その階層で最後かを判定
        let result_last = &self
            .last_infos
            .iter()
            .position(|last| last.name == target_dname);
        match result_last {
            Some(i) => {
                self.last_infos.get_mut(*i).unwrap().flg = true;
                Some(true)
            }
            None => None,
        };

        // ファイル一覧分処理を実行
        for ent in files {
            // パスからファイル名を取り出し
            let current_path = ent.path();
            let current_fname = current_path.file_name().unwrap().to_string_lossy();

            // 除外ファイル判定
            let result = &self.exclusions.iter().position(|x| x == &current_fname);
            match result {
                Some(_) => continue,
                None => 0,
            };

            // 隠しファイル除外判定
            let re = Regex::new(r"^\..").unwrap();
            if re.is_match(&current_fname) {
                continue;
            }

            // 階層出力
            for current_level in 1..=level {
                // 現在処理中のディレクトリ又はファイルが最終レベルかを判定
                let result_last_child = &self
                    .last_infos
                    .iter()
                    .position(|info| info.level == current_level && info.flg);
                let last_flg = match result_last_child {
                    Some(_) => true,
                    None => false,
                };

                if last_flg {
                    print!("    ");
                } else {
                    print!("│   ");
                }
            }

            // tree出力
            if current_fname.eq_ignore_ascii_case(&last_fname) {
                println!("└── {}", current_fname);
            } else {
                println!("├── {}", current_fname);
            }

            // 対象のパスがファイルの場合
            if current_path.is_file() {
                self.f_cnt += 1;
            }

            // 対象のパスがディレクトリの場合
            if current_path.is_dir() {
                self.d_cnt += 1;

                // 更に下位階層へ(再帰呼び出し)
                self.run(&current_path, level + 1);
            }
        }
    }

    /// ルートディレクトリを出力
    ///
    /// * `target_dir` - 対象ディレクトリ
    pub fn print_current_dir(&mut self, target_dir: &str) {
        self.d_cnt += 1;
        let result = if self.current_flg {
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

    /// 各階層の最終ディレクトリを表示
    pub fn print_last(&self) {
        for last in self.last_infos.iter() {
            println!(
                "level: {} name: {} flg: {}",
                last.level, last.name, last.flg
            );
        }
    }
}
