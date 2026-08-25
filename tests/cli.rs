use std::process::Command;

#[test]
fn binary_version_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_myip"))
        .arg("--version")
        .output()
        .expect("run myip --version");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, format!("myip {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn binary_rejects_unknown_argument() {
    let output = Command::new(env!("CARGO_BIN_EXE_myip"))
        .arg("--nope")
        .output()
        .expect("run myip --nope");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unexpected argument: --nope"));
}
