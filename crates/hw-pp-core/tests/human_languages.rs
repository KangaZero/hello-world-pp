use hw_pp_core::data::human_languages::{
    DialectName, HumanLanguageError, HumanLanguageName, get_all_human_language_names,
    hello_world_for,
};

#[test]
fn english_american_hello_world() {
    assert_eq!(
        hello_world_for(
            Some(HumanLanguageName::English),
            Some(DialectName::AmericanEnglish)
        ),
        Ok("Hello, World!")
    );
}

#[test]
fn kansai_japanese_hello_world() {
    assert_eq!(
        hello_world_for(
            Some(HumanLanguageName::Japanese),
            Some(DialectName::KansaiJapanese)
        ),
        Ok("まいど、世界！")
    );
}

#[test]
fn english_katakana_hello_world() {
    assert_eq!(
        hello_world_for(
            Some(HumanLanguageName::English),
            Some(DialectName::EnglishKatakana)
        ),
        Ok("ハローワールド")
    );
}

#[test]
fn dialect_mismatch_for_language_errors() {
    let expected = HumanLanguageError::UnknownDialect {
        language: HumanLanguageName::Spanish,
        dialect: DialectName::Mandarin,
    };
    assert_eq!(
        hello_world_for(
            Some(HumanLanguageName::Spanish),
            Some(DialectName::Mandarin)
        ),
        Err(expected)
    );
}

#[test]
fn all_human_language_names() {
    let result = get_all_human_language_names();
    assert!(result.contains(&HumanLanguageName::English));
    assert!(result.len() > 20);
}
