use assert_cmd::Command;

#[test]
fn no_args_prints_default_hello_world() {
    Command::cargo_bin("hw-pp")
        .expect("binary exists")
        .assert()
        .success()
        .stdout("Hello, World!\n");
}

#[test]
fn human_language_long_flag_selects_language() {
    Command::cargo_bin("hw-pp")
        .expect("binary exists")
        .args(["--human-language", "Japanese"])
        .assert()
        .success()
        .stdout("こんにちは世界！\n");
}

#[test]
fn human_language_hl_alias_and_short_flag_work() {
    for flag in ["--hl", "-l"] {
        Command::cargo_bin("hw-pp")
            .expect("binary exists")
            .args([flag, "german"])
            .assert()
            .success()
            .stdout("Hallo, Welt!\n");
    }
}

#[test]
fn unknown_human_language_is_rejected() {
    Command::cargo_bin("hw-pp")
        .expect("binary exists")
        .args(["--hl", "Elvish"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            r#"unrecognized language name: "Elvish""#,
        ));
}
