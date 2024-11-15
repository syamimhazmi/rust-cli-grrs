use std::{error::Error, fs};
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn create_temp_file() -> Result<(NamedTempFile, Command), Box<dyn Error>> {
    let temp_file = NamedTempFile::new()?;
    fs::write(&temp_file, "Hello World\nRust is awesome\nHello Rust\n")?;
    let cmd = Command::cargo_bin("cli_grrs")?;
    
    Ok((temp_file, cmd))
}

#[test]
fn file_doesnt_exists_should_return_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli_grrs")?;

    cmd.arg("foobar")
        .arg("test/file/doesnt/exists")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to open file"));

    Ok(())
}

#[test]
fn able_to_get_matches_string() -> Result<(), Box<dyn std::error::Error>> {
    let (temp_file, mut cmd) = create_temp_file()?;

    cmd.arg("Hello")
        .arg(temp_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World\nHello Rust\n"));

    Ok(())
}

#[test]
fn empty_pattern_should_produce_error() -> Result<(), Box<dyn std::error::Error>> {
    let (temp_file, mut cmd) = create_temp_file()?;

    cmd.arg("")
        .arg(temp_file.path())
        .assert()
        .failure()  
        .code(1)
        .stderr(predicate::str::contains("unexpected argument '' found"));

    Ok(())
}
