use assert_cmd::Command;
use predicates::prelude::*;

fn hw_pp() -> Command {
    Command::cargo_bin("hw-pp").expect("binary exists")
}

/// One valid `(long flag, value)` per option, so a failure can only be the conflict.
const HUMAN_OPTIONS: [(&str, &str); 3] = [
    ("--human-language", "english"),
    ("--human-dialect", "american english"),
    ("--script", "latin"),
];
const PROGRAMMING_OPTIONS: [(&str, &str); 3] = [
    ("--programming-language", "rust"),
    ("--programming-dialect", "rust 2021"),
    ("--extension", "rs"),
];

#[test]
fn no_args_prints_default_hello_world() {
    hw_pp().assert().success().stdout("Hello, World!\n");
}

#[test]
fn human_language_long_flag_selects_language() {
    hw_pp()
        .args(["--human-language", "Japanese"])
        .assert()
        .success()
        .stdout("こんにちは世界！\n");
}

#[test]
fn human_language_hl_alias_and_short_flag_work() {
    for flag in ["--hl", "-l"] {
        hw_pp()
            .args([flag, "german"])
            .assert()
            .success()
            .stdout("Hallo, Welt!\n");
    }
}

#[test]
fn unknown_human_language_is_rejected() {
    hw_pp()
        .args(["--hl", "Elvish"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            r#"unrecognized language name: "Elvish""#,
        ));
}

#[test]
fn programming_options_print_hello_world_syntax() {
    hw_pp()
        .args(["--pd", "Python 3"])
        .assert()
        .success()
        .stdout("print(\"Hello, World!\")\n");
    hw_pp()
        .args(["-e", "rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("println!(\"Hello, World!\");"));
}

#[test]
fn every_human_option_combines_with_every_programming_option() {
    for (human_flag, human_value) in HUMAN_OPTIONS {
        for (programming_flag, programming_value) in PROGRAMMING_OPTIONS {
            let human = [human_flag, human_value];
            let programming = [programming_flag, programming_value];
            for args in [[human, programming].concat(), [programming, human].concat()] {
                hw_pp()
                    .args(&args)
                    .assert()
                    .success()
                    .stdout(predicate::str::contains(r#"println!("Hello, World!");"#));
            }
        }
    }
}

#[test]
fn short_flags_and_aliases_also_combine() {
    for args in [
        ["-l", "german", "-p", "rust"],
        ["--hl", "german", "--pl", "rust"],
        ["--hd", "standard german", "-e", "rs"],
        ["-l", "german", "--pd", "rust 2021"],
    ] {
        hw_pp()
            .args(args)
            .assert()
            .success()
            .stdout(predicate::str::contains(r#"println!("Hallo, Welt!");"#));
    }
}

#[test]
fn no_punctuation_strips_only_the_inserted_greeting() {
    hw_pp()
        .args(["-l", "german", "-p", "rust", "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"println!("Hallo Welt");"#));
}

#[test]
fn inserting_into_a_snippet_without_the_message_fails() {
    hw_pp()
        .args(["-l", "german", "-p", "brainfuck"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            r#"snippet does not contain "Hello, World!""#,
        ));
}

#[test]
fn all_options_in_one_group_combine_without_conflict() {
    hw_pp()
        .args([
            "--hl",
            "english",
            "--hd",
            "american english",
            "-s",
            "katakana",
        ])
        .assert()
        .success()
        .stdout("ハローワールド！\n");
    hw_pp()
        .args(["-p", "python", "--pd", "python 2", "-e", "py"])
        .assert()
        .success()
        .stdout("print \"Hello, World!\"\n");
}

#[test]
fn no_punctuation_only_strips_string_literals_in_code() {
    hw_pp()
        .args(["-p", "rust", "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"println!("Hello World");"#));
}

#[test]
fn piped_programming_output_has_no_ansi_codes() {
    hw_pp()
        .args(["-p", "rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains('\u{1b}').not());
}
