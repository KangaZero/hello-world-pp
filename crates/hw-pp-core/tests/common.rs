use hw_pp_core::data::common::{
    DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT, DEFAULT_PROGRAMMING_LANGUAGE,
    HELLO_WORLD_PUNCTUATION, HelloWorldNotFoundError, get_default_human_language_hello_world,
    remove_punctuation_from_hello_world, replace_default_hello_world_string_with,
};
use hw_pp_core::data::human_languages::{DialectName, HUMAN_LANGUAGES, HumanLanguageName};
use hw_pp_core::data::programming_languages::{
    ProgrammingLanguageName, get_hello_world_syntax_for,
};

#[test]
fn defaults_are_english_c_and_latin() {
    assert_eq!(DEFAULT_HUMAN_LANGUAGE, HumanLanguageName::English);
    assert_eq!(DEFAULT_HUMAN_LANGUAGE_DIALECT, DialectName::AmericanEnglish);
    assert_eq!(DEFAULT_PROGRAMMING_LANGUAGE, ProgrammingLanguageName::C);
    assert_eq!(get_default_human_language_hello_world(), "Hello, World!");
}

#[test]
fn remove_punctuation_strips_ascii_and_cjk_marks() {
    assert_eq!(
        remove_punctuation_from_hello_world("Hello, World!"),
        "Hello World"
    );
    assert_eq!(
        remove_punctuation_from_hello_world("¡Hola, Mundo!"),
        "Hola Mundo"
    );
    assert_eq!(
        remove_punctuation_from_hello_world("你好，世界！"),
        "你好世界"
    );
    assert_eq!(
        remove_punctuation_from_hello_world("まいど、世界！"),
        "まいど世界"
    );
}

#[test]
fn remove_punctuation_keeps_apostrophes_and_braille() {
    assert_eq!(
        remove_punctuation_from_hello_world("nuqneH, qo'!"),
        "nuqneH qo'"
    );
    assert_eq!(
        remove_punctuation_from_hello_world("⠠⠓⠑⠇⠇⠕⠂ ⠠⠺⠕⠗⠇⠙⠖"),
        "⠠⠓⠑⠇⠇⠕⠂ ⠠⠺⠕⠗⠇⠙⠖"
    );
}

/// Fails when a new greeting introduces a punctuation mark the list does not cover.
#[test]
fn hello_world_punctuation_covers_every_greeting() {
    const BRAILLE: core::ops::RangeInclusive<char> = '\u{2800}'..='\u{28FF}';
    /// Signs that are neither letters to `char::is_alphanumeric` nor punctuation.
    const VIRAMAS: &[char] = &['\u{094D}', '\u{09CD}', '\u{0BCD}', '\u{0C4D}'];

    let uncovered: Vec<String> = HUMAN_LANGUAGES
        .iter()
        .flat_map(|lang| lang.dialects)
        .flat_map(|dialect| dialect.greetings)
        .filter_map(|greeting| {
            let chars: String = greeting
                .text
                .chars()
                .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
                .filter(|c| *c != '\'' && !BRAILLE.contains(c) && !VIRAMAS.contains(c))
                .filter(|c| !HELLO_WORLD_PUNCTUATION.contains(c))
                .collect();
            (!chars.is_empty()).then(|| format!("{chars:?} in {:?}", greeting.text))
        })
        .collect();
    assert!(uncovered.is_empty(), "uncovered characters: {uncovered:#?}");
}

#[test]
fn replace_hello_world_swaps_in_a_human_greeting() {
    let rust = get_hello_world_syntax_for(Some(ProgrammingLanguageName::Rust), None, None)
        .expect("Rust snippet exists");
    let replaced = replace_default_hello_world_string_with(rust, "Hallo, Welt!")
        .expect("Rust snippet contains the message");
    assert!(replaced.contains(r#"println!("Hallo, Welt!");"#));
    assert!(!replaced.contains("Hello, World!"));
}

#[test]
fn replace_hello_world_errors_when_message_is_absent() {
    let brainfuck =
        get_hello_world_syntax_for(Some(ProgrammingLanguageName::Brainfuck), None, None)
            .expect("Brainfuck snippet exists");
    let err = replace_default_hello_world_string_with(brainfuck, "Hallo, Welt!").unwrap_err();
    assert_eq!(err, HelloWorldNotFoundError);
    assert_eq!(
        err.to_string(),
        r#"snippet does not contain "Hello, World!""#
    );
}
