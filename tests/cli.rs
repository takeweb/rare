use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rtree";
const DIR_PTN1: &str = "./tests/inputs/inputs1";
const DIR_PTN2: &str = "./tests/inputs/inputs2";
const DIR_PTN3: &str = "./tests";
const EXP_PTN1: &str = "tests/expected/one_args_ptn1.txt";
const EXP_PTN2: &str = "tests/expected/one_args_ptn2.txt";
const EXP_PTN3: &str = "tests/expected/one_args_ptn3.txt";

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
