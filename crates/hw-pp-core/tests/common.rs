use hw_pp_core::data::common::{
    DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT, DEFAULT_PROGRAMMING_LANGUAGE,
    HELLO_WORLD_PUNCTUATION, get_default_human_language_hello_world, remove_punctuation_from_str,
    replace_default_hello_world_string_with, reverse_str,
};
use hw_pp_core::data::human_languages::{DialectName, HUMAN_LANGUAGES, HumanLanguageName};
use hw_pp_core::data::programming_languages::{
    ProgrammingLanguageDialect, ProgrammingLanguageName, get_hello_world_syntax_for,
};
use unicode_normalization::char::is_combining_mark;

#[test]
fn defaults_are_american_english_and_c() {
    assert_eq!(DEFAULT_HUMAN_LANGUAGE, HumanLanguageName::English);
    assert_eq!(DEFAULT_HUMAN_LANGUAGE_DIALECT, DialectName::AmericanEnglish);
    assert_eq!(DEFAULT_PROGRAMMING_LANGUAGE, ProgrammingLanguageName::C);
    assert_eq!(get_default_human_language_hello_world(), "Hello, World!");
}

#[test]
fn remove_punctuation_strips_ascii_and_cjk_marks() {
    assert_eq!(remove_punctuation_from_str("Hello, World!"), "Hello World");
    assert_eq!(remove_punctuation_from_str("¡Hola, Mundo!"), "Hola Mundo");
    assert_eq!(remove_punctuation_from_str("你好，世界！"), "你好世界");
    assert_eq!(remove_punctuation_from_str("まいど、世界！"), "まいど世界");
}

#[test]
fn remove_punctuation_keeps_apostrophes_and_braille() {
    assert_eq!(remove_punctuation_from_str("nuqneH, qo'!"), "nuqneH qo'");
    assert_eq!(
        remove_punctuation_from_str("⠠⠓⠑⠇⠇⠕⠂ ⠠⠺⠕⠗⠇⠙⠖"),
        "⠠⠓⠑⠇⠇⠕⠂ ⠠⠺⠕⠗⠇⠙⠖"
    );
}

/// Fails when a new greeting introduces a punctuation mark the list does not cover.
#[test]
fn hello_world_punctuation_covers_every_greeting() {
    const BRAILLE: core::ops::RangeInclusive<char> = '\u{2800}'..='\u{28FF}';
    /// Word-internal marks: the apostrophe, hyphens (including the Hebrew maqaf `־`), the
    /// Tibetan syllable separator (tsheg) and the morpheme dot of Egyptian transliteration (`j.nḏ`).
    const WORD_MARKS: &[char] = &['\'', '-', '.', '\u{05BE}', '\u{0F0B}'];

    let uncovered: Vec<String> = HUMAN_LANGUAGES
        .iter()
        .flat_map(|lang| lang.dialects)
        .flat_map(|dialect| dialect.greetings)
        .filter_map(|greeting| {
            let chars: String = greeting
                .text
                .chars()
                .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
                // Combining marks (viramas, vowel signs) are part of the letters around them.
                .filter(|c| {
                    !is_combining_mark(*c) && !WORD_MARKS.contains(c) && !BRAILLE.contains(c)
                })
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
    let replaced = replace_default_hello_world_string_with(rust, "Hallo, Welt!");
    assert!(replaced.contains(r#"println!("Hallo, Welt!");"#));
    assert!(!replaced.contains("Hello, World!"));
}

#[test]
fn replace_hello_world_reverses_the_greeting_for_a_reversed_message() {
    let befunge = get_hello_world_syntax_for(
        Some(ProgrammingLanguageName::Befunge),
        Some(ProgrammingLanguageDialect::Befunge93),
        None,
    )
    .expect("Befunge-93 snippet exists");
    let replaced = replace_default_hello_world_string_with(befunge, "Hallo, Welt!");
    assert_eq!(replaced, r#""!tleW ,ollaH">:#,_@"#);
}

#[test]
fn replace_hello_world_returns_encoded_snippets_unchanged() {
    let brainfuck =
        get_hello_world_syntax_for(Some(ProgrammingLanguageName::Brainfuck), None, None)
            .expect("Brainfuck snippet exists");
    assert_eq!(
        replace_default_hello_world_string_with(brainfuck, "Hallo, Welt!"),
        brainfuck
    );
}

#[test]
fn replace_hello_world_returns_ambiguous_snippets_unchanged() {
    // Both directions at once: replacing either would leave the other stale.
    let both = r#"print("Hello, World!"); print("!dlroW ,olleH")"#;
    assert_eq!(
        replace_default_hello_world_string_with(both, "Hallo, Welt!"),
        both
    );
}

#[test]
fn reverse_str_keeps_grapheme_clusters_whole() {
    assert_eq!(reverse_str("Hello, World!"), "!dlroW ,olleH");
    // "é" as "e" + combining acute: reversing by `char` would put the accent on the "b".
    assert_eq!(reverse_str("Que\u{301}bec"), "cebe\u{301}uQ");
    // A flag is two regional-indicator code points that must stay in order.
    assert_eq!(reverse_str("Hi 🇩🇪"), "🇩🇪 iH");
}
