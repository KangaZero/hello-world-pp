use assert_cmd::Command;
use hw_pp_core::data::human_languages::{Greeting, HumanLanguageName};
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
/// One valid `(long flag, value)` per option, so a failure can only be the conflict.
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
        .args(["-l", "german", "-p", "rust", "--no-punctuation"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"println!("Hallo Welt");"#));
}

#[test]
fn inserting_into_an_encoded_snippet_warns_and_prints_it_unchanged() {
    hw_pp()
        .args(["-l", "german", "-p", "brainfuck", "--color", "no"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("++++++++"))
        .stderr(predicate::str::contains(
            r#"warning: this snippet encodes its message, so "Hallo, Welt!" could not be inserted"#,
        ));
}

#[test]
fn inserting_into_a_reversed_message_reverses_the_greeting() {
    hw_pp()
        .args(["-l", "german", "-p", "befunge", "--color", "no"])
        .assert()
        .success()
        .stdout("\"!tleW ,ollaH\">:#,_@\n")
        .stderr("");
}

#[test]
fn inserting_the_default_greeting_does_not_warn() {
    hw_pp()
        .args(["-l", "english", "-p", "rust"])
        .assert()
        .success()
        .stderr("");
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
fn no_punctuation_only_strips_the_message_in_code() {
    hw_pp()
        .args(["-p", "rust", "--no-punctuation"])
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

#[test]
fn hyphenated_names_parse_like_spaced_ones() {
    for name in ["Bahasa Indonesia", "bahasa-indonesia", "bahasa_indonesia"] {
        hw_pp()
            .args(["-l", name])
            .assert()
            .success()
            .stdout("Halo, Dunia!\n");
    }
}

#[test]
fn list_prints_multi_word_names_hyphenated() {
    hw_pp()
        .args(["list", "--compact", "themes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Catppuccin-Latte"))
        .stdout(predicate::str::contains("Catppuccin Latte").not());
}

#[test]
fn list_filters_human_languages_by_family() {
    hw_pp()
        .args(["list", "hl", "--family", "bantu"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Zulu\n"))
        .stdout(predicate::str::contains("English").not());
    hw_pp()
        .args(["list", "hf", "english"])
        .assert()
        .success()
        .stdout("Germanic\nIndo-European\n");
}

#[test]
fn list_filters_programming_languages_by_paradigm() {
    hw_pp()
        .args(["list", "pl", "--paradigm", "logic"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Prolog\n"))
        .stdout(predicate::str::contains("\nC\n").not());
    hw_pp()
        .args(["list", "pp", "prolog"])
        .assert()
        .success()
        .stdout("Declarative\nLogic\n");
}

#[test]
fn list_notations_includes_every_notation() {
    hw_pp()
        .args(["list", "--count", "notations"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Base64\n"))
        .stdout(predicate::str::contains("Code Points\n"))
        .stdout(predicate::str::contains(format!(
            "Count: {}\n",
            hw_pp_core::data::notation::Notation::ALL.len()
        )));
}

#[test]
fn dialect_parses_with_any_accent_case_or_separator() {
    for name in [
        "Québécois French",
        "quebecois french",
        "QUEBECOIS-FRENCH",
        "québécois_french",
        "francais-quebecois",
    ] {
        hw_pp()
            .args(["--human-dialect", name])
            .assert()
            .success()
            .stdout("Bonjour le monde!\n");
    }
}

#[test]
fn script_conflicts_with_nato_notation() {
    hw_pp()
        .args(["-s", "katakana", "--notation", "nato"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "cannot be used with '--notation nato'",
        ));
}

#[test]
fn script_combines_with_other_notations() {
    hw_pp()
        .args(["-s", "katakana", "--notation", "upper-hex"])
        .assert()
        .success();
}

#[test]
fn nato_picks_the_latin_script_of_the_language() {
    hw_pp()
        .args(["-l", "japanese", "--notation", "nato"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("Kilo Oscar November November"))
        .stderr("");
}

#[test]
fn nato_warns_when_the_language_has_no_latin_script() {
    let asl = Greeting::text_for(Some(HumanLanguageName::AmericanSignLanguage), None, None)
        .expect("ASL has a greeting");
    hw_pp()
        .args(["-l", "american sign language", "--notation", "nato"])
        .assert()
        .success()
        // The ASCII-art handshapes contain no letters, so NATO leaves them as they are.
        .stdout(format!("{asl}\n"))
        .stderr(predicate::str::contains(
            "warning: this language has no Latin-script greeting",
        ));
}

#[test]
fn nato_encodes_only_the_message_in_snippets() {
    hw_pp()
        .args(["-p", "rust", "-c", "no", "--notation", "nato"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"println!("Hotel Echo Lima Lima Oscar, Whiskey Oscar Romeo Lima Delta!");"#,
        ));
}
