use hw_pp_core::data::human_languages::{
    DialectName, HUMAN_LANGUAGES, HumanLanguageCategory, HumanLanguageError, HumanLanguageName,
    ScriptName, get_all_human_languages_as_str_by, get_human_language_from_dialect,
    get_human_language_hello_world_for,
};
use hw_pp_core::data::lookup::LookupError;

#[test]
fn english_american_hello_world() {
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::English),
            Some(DialectName::AmericanEnglish),
            None
        ),
        Ok("Hello, World!")
    );
}

#[test]
fn kansai_japanese_hello_world() {
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::Japanese),
            Some(DialectName::KansaiJapanese),
            None
        ),
        Ok("まいど、世界！")
    );
}

#[test]
fn script_selects_greeting_within_dialect() {
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::English),
            None,
            Some(ScriptName::Katakana)
        ),
        Ok("ハローワールド")
    );
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::Japanese),
            Some(DialectName::KansaiJapanese),
            Some(ScriptName::Romaji)
        ),
        Ok("maido, sekai!")
    );
}

#[test]
fn unrecorded_script_errors() {
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::English),
            Some(DialectName::BritishEnglish),
            Some(ScriptName::Katakana)
        ),
        Err(HumanLanguageError::UnknownScript {
            language: HumanLanguageName::English,
            dialect: DialectName::BritishEnglish,
            script: ScriptName::Katakana,
        })
    );
}

#[test]
fn every_dialect_has_a_greeting_in_declared_language_scripts() {
    for lang in HUMAN_LANGUAGES {
        for dialect in lang.dialects {
            assert!(
                !dialect.greetings.is_empty(),
                "{} has no greeting",
                dialect.label
            );
            for greeting in dialect.greetings {
                for script in greeting.scripts {
                    assert!(
                        lang.scripts.contains(script),
                        "{} uses {script}, which {} does not list in `scripts`",
                        dialect.label,
                        lang.name
                    );
                }
            }
        }
    }
}

#[test]
fn dialect_mismatch_for_language_errors() {
    let expected = HumanLanguageError::Lookup(LookupError::UnknownVariant {
        language: HumanLanguageName::Spanish,
        variant: DialectName::Mandarin,
    });
    assert_eq!(
        get_human_language_hello_world_for(
            Some(HumanLanguageName::Spanish),
            Some(DialectName::Mandarin),
            None
        ),
        Err(expected)
    );
}

#[test]
fn all_human_language_names() {
    let result = get_all_human_languages_as_str_by(None, None);
    assert!(result.contains(&"English"));
    assert_eq!(result.len(), HumanLanguageName::ALL.len());
}

#[test]
fn every_language_name_round_trips_through_from_str() {
    for &name in HumanLanguageName::ALL {
        assert_eq!(name.as_str().parse::<HumanLanguageName>(), Ok(name));
    }
}

#[test]
fn language_name_parse_ignores_ascii_case_and_whitespace() {
    assert_eq!(
        "  ancient greek ".parse::<HumanLanguageName>(),
        Ok(HumanLanguageName::AncientGreek)
    );
}

#[test]
fn unknown_language_name_errors_with_input() {
    let err = "Elvish".parse::<HumanLanguageName>().unwrap_err();
    assert_eq!(err.input(), "Elvish");
    assert_eq!(err.to_string(), r#"unrecognized language name: "Elvish""#);
}

#[test]
fn dialect_without_language_infers_language() {
    assert_eq!(
        get_human_language_hello_world_for(None, Some(DialectName::Mandarin), None),
        Ok("你好，世界！")
    );
}

#[test]
fn every_dialect_belongs_to_exactly_one_language() {
    for lang in HUMAN_LANGUAGES {
        for dialect in lang.dialects {
            let owners = HUMAN_LANGUAGES
                .iter()
                .filter(|other| other.dialects.iter().any(|d| d.label == dialect.label))
                .count();
            assert_eq!(
                owners, 1,
                "{} is recorded under {owners} languages",
                dialect.label
            );
            assert_eq!(
                get_human_language_from_dialect(dialect.label),
                Ok(lang.name)
            );
        }
    }
}

#[test]
fn every_name_enum_round_trips_through_display_and_from_str() {
    fn assert_round_trips<T>(all: &[T])
    where
        T: Copy + PartialEq + core::fmt::Debug + core::fmt::Display + core::str::FromStr,
        T::Err: core::fmt::Debug,
    {
        for &variant in all {
            let text = variant.to_string();
            assert_eq!(
                text.parse::<T>().ok(),
                Some(variant),
                "{text:?} did not round-trip"
            );
            assert_eq!(text.to_ascii_lowercase().parse::<T>().ok(), Some(variant));
        }
    }
    assert_round_trips(HumanLanguageCategory::ALL);
    assert_round_trips(ScriptName::ALL);
    assert_round_trips(HumanLanguageName::ALL);
    assert_round_trips(DialectName::ALL);
}

#[test]
fn every_variant_is_used_in_the_data() {
    for name in HumanLanguageName::ALL {
        assert!(
            HUMAN_LANGUAGES.iter().any(|l| l.name == *name),
            "{name} has no entry"
        );
    }
    for dialect in DialectName::ALL {
        assert!(
            HUMAN_LANGUAGES
                .iter()
                .any(|l| l.dialects.iter().any(|d| d.label == *dialect)),
            "{dialect} has no entry"
        );
    }
    for script in ScriptName::ALL {
        assert!(
            HUMAN_LANGUAGES.iter().any(|l| l.scripts.contains(script)),
            "{script} is not listed by any language"
        );
    }
}

#[test]
fn script_and_category_parse_errors_name_their_kind() {
    let err = "Tengwar".parse::<ScriptName>().unwrap_err();
    assert_eq!(err.to_string(), r#"unrecognized script name: "Tengwar""#);
    assert_eq!(err.input(), "Tengwar");
    let err = "Fictional".parse::<HumanLanguageCategory>().unwrap_err();
    assert_eq!(
        err.to_string(),
        r#"unrecognized category name: "Fictional""#
    );
}

#[test]
fn lookup_errors_have_readable_messages() {
    let err = get_human_language_hello_world_for(
        Some(HumanLanguageName::Spanish),
        Some(DialectName::Mandarin),
        None,
    )
    .unwrap_err();
    assert_eq!(err.to_string(), "Mandarin is not recorded for Spanish");
}
