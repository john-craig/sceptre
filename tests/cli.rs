use std::process::Command;

#[test]
fn binary_prints_a_greeting() {
    let bin = env!("CARGO_BIN_EXE_rust-template");
    let output = Command::new(bin)
        .arg("Ada")
        .output()
        .expect("binary to run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("Hello, Ada."));
}
