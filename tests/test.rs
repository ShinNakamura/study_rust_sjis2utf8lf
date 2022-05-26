use assert_cmd::Command;
use std::fs;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_main() -> TestResult {
    let textdata_path = Path::new("tests").join("textdata");
    let expected_path = textdata_path.join("expested.txt");
    let expected = fs::read_to_string(expected_path)?;
    let mut cmd = Command::cargo_bin("sjis2utf8lf")?;
    let input_path = textdata_path.join("input.sjis.crlf.txt");
    cmd.arg(input_path)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
