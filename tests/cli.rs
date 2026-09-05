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

#[test]
fn repository_create_exposes_https_override() {
    let bin = env!("CARGO_BIN_EXE_rust-template");
    let output = Command::new(bin)
        .args(["repository", "create", "--help"])
        .output()
        .expect("binary to run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("--https"));
}

#[test]
fn idea_create_help_exposes_machine_readable_inputs() {
    let bin = env!("CARGO_BIN_EXE_rust-template");
    let output = Command::new(bin)
        .args(["idea", "create", "--help"])
        .output()
        .expect("binary to run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    for argument in [
        "--repository",
        "--title",
        "--problem",
        "--desired-outcome",
        "--open-questions",
    ] {
        assert!(stdout.contains(argument), "missing {argument}");
    }
}

#[test]
fn workflow_command_families_expose_json_workflow_operations() {
    let bin = env!("CARGO_BIN_EXE_rust-template");
    for (family, commands) in [
        (
            "idea-process",
            ["pending", "status", "prepare", "publish", "feedback"],
        ),
        (
            "integration",
            ["candidates", "status", "merge", "sync", "cleanup"],
        ),
    ] {
        let output = Command::new(bin)
            .args([family, "--help"])
            .output()
            .expect("binary to run");
        assert!(output.status.success());
        let help = String::from_utf8(output.stdout).expect("utf8 help");
        for command in commands {
            assert!(help.contains(command), "missing {family} {command}");
        }
    }
}

#[test]
fn specset_publish_requires_implementation_path_option() {
    let bin = env!("CARGO_BIN_EXE_rust-template");
    let output = Command::new(bin)
        .args(["specset", "publish", "--help"])
        .output()
        .expect("binary to run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 help");
    assert!(stdout.contains("--implementation-path <PATH>"));
}
