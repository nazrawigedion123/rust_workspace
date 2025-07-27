use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .stdout(predicates::str::contains(" Meow!"))
        .success();
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}
