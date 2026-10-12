use assert_cmd::Command;

#[test]
fn count_zero_prints_default_hello_world() {
    Command::cargo_bin("hw-pp")
        .expect("binary exists")
        .args(["--name", "Samuel", "--count", "0"])
        .assert()
        .success()
        .stdout("Hello, World!\n");
}

#[test]
fn default_greets_the_name() {
    Command::cargo_bin("hw-pp")
        .expect("binary exists")
        .args(["--name", "Samuel"])
        .assert()
        .success()
        .stdout("Hello Samuel!\n");
}
