use hw_pp_core::data::human_languages::{
    DialectName, HumanLanguageError, HumanLanguageName, greeting_for, greetings_for,
};

#[test]
fn english_american_hello_world() {
    assert_eq!(
        greeting_for(HumanLanguageName::English, DialectName::AmericanEnglish),
        Ok("Hello, World!")
    );
}

#[test]
fn kansai_japanese_greetings() {
    assert_eq!(
        greetings_for(HumanLanguageName::Japanese, DialectName::KansaiJapanese),
        Ok(&["こんにちは世界！", "まいど、世界！"][..])
    );
}

#[test]
fn english_katakana_greetings() {
    assert_eq!(
        greetings_for(HumanLanguageName::English, DialectName::EnglishKatakana),
        Ok(&["ハローワールド", "ハーロ、ワルド"][..])
    );
}

#[test]
fn dialect_mismatch_for_language_errors() {
    let expected = HumanLanguageError::UnknownDialect {
        language: HumanLanguageName::Spanish,
        dialect: DialectName::Mandarin,
    };
    assert_eq!(
        greeting_for(HumanLanguageName::Spanish, DialectName::Mandarin),
        Err(expected)
    );
    assert_eq!(
        greetings_for(HumanLanguageName::Spanish, DialectName::Mandarin),
        Err(expected)
    );
}
