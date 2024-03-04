use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rtree";
const DIR_PTN1: &str = "./tests/inputs/inputs1";
const DIR_PTN2: &str = "./tests/inputs/inputs2";
const DIR_PTN3: &str = "./tests";

// --------------------------------------------------
#[test]
fn one_args_ptn1() -> TestResult {
    let expected = "tests/expected/one_args_ptn1.txt";
    run(&[DIR_PTN1], expected)
}

#[test]
fn one_args_ptn2() -> TestResult {
    let expected = "tests/expected/one_args_ptn2.txt";
    run(&[DIR_PTN2], expected)
}

#[test]
fn one_args_ptn3() -> TestResult {
    let expected = "tests/expected/one_args_ptn3.txt";
    run(&[DIR_PTN3], expected)
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
