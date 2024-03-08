use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rtree";
const DIR_PTN1: &str = "./tests/inputs/input1";
const DIR_PTN2: &str = "./tests/inputs/input2";
const DIR_PTN3: &str = "./tests/inputs/input3";
const DIR_PTN4: &str = "./tests/inputs";
const DIR_PTN5: &str = "./tests";
// const DIR_PTN0: &str = ".";
const EXP_PTN1: &str = "tests/expected/one_args_ptn1.txt";
const EXP_PTN2: &str = "tests/expected/one_args_ptn2.txt";
const EXP_PTN3: &str = "tests/expected/one_args_ptn3.txt";
const EXP_PTN4: &str = "tests/expected/one_args_ptn4.txt";
const EXP_PTN5: &str = "tests/expected/one_args_ptn5.txt";
// const EXP_PTN9: &str = "tests/expected/one_args_ptn9.txt";

// --------------------------------------------------
#[test]
fn one_args_ptn1() -> TestResult {
    run(&[DIR_PTN1], EXP_PTN1)
}

#[test]
fn one_args_ptn2() -> TestResult {
    run(&[DIR_PTN2], EXP_PTN2)
}

#[test]
fn one_args_ptn3() -> TestResult {
    run(&[DIR_PTN3], EXP_PTN3)
}

#[test]
fn one_args_ptn4() -> TestResult {
    run(&[DIR_PTN4], EXP_PTN4)
}

#[test]
fn one_args_ptn5() -> TestResult {
    run(&[DIR_PTN5], EXP_PTN5)
}

// cargo testで実行すると、デバッグ情報等の出力されるファイルが毎回変更されてしまう為、再現テストは不可
// #[test]
// fn one_args_ptn9() -> TestResult {
//     run(&[DIR_PTN9], EXP_PTN9)
// }

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
