use hw_pp_core::data::human_languages::{
    DialectName, Greeting, HUMAN_LANGUAGES, HumanLanguageCategory, HumanLanguageError,
    HumanLanguageFamily, HumanLanguageName, ScriptName,
};
use hw_pp_core::data::lookup::LookupError;

#[test]
fn english_american_hello_world() {
    assert_eq!(
        Greeting::text_for(
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
        Greeting::text_for(
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
        Greeting::text_for(
            Some(HumanLanguageName::English),
            None,
            Some(ScriptName::Katakana)
        ),
        Ok("ハローワールド！")
    );
    assert_eq!(
        Greeting::text_for(
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
        Greeting::text_for(
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
        Greeting::text_for(
            Some(HumanLanguageName::Spanish),
            Some(DialectName::Mandarin),
            None
        ),
        Err(expected)
    );
}

#[test]
fn all_human_language_names() {
    let result = HumanLanguageName::all_as_str_by(None, None, None);
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
        Greeting::text_for(None, Some(DialectName::Mandarin), None),
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
            assert_eq!(dialect.label.language(), Ok(lang.name));
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
    assert_round_trips(HumanLanguageFamily::ALL);
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
fn families_filter_languages_and_list_per_language() {
    let bantu = HumanLanguageName::all_as_str_by(None, None, Some(HumanLanguageFamily::Bantu));
    assert!(bantu.contains(&"Zulu") && bantu.contains(&"Swahili"));
    assert!(!bantu.contains(&"Yoruba"));
    assert_eq!(
        HumanLanguageFamily::all_as_str_by(Some(HumanLanguageName::Spanish)),
        ["Indo-European", "Italic", "Romance"]
    );
    assert_eq!(
        HumanLanguageFamily::all_as_str_by(Some(HumanLanguageName::Basque)),
        Vec::<&str>::new()
    );
}

#[test]
fn every_branch_is_listed_with_its_family() {
    use HumanLanguageFamily::{
        Afroasiatic, Austronesian, Baltic, Bantu, Berber, Celtic, Chadic, Cushitic, Egyptian,
        Germanic, Hellenic, IndoAryan, IndoEuropean, Iranian, Italic, NigerCongo, Polynesian,
        Romance, Semitic, Sinitic, SinoTibetan, Slavic,
    };
    let parents = [
        (Germanic, IndoEuropean),
        (Italic, IndoEuropean),
        (Romance, Italic),
        (Celtic, IndoEuropean),
        (Slavic, IndoEuropean),
        (Baltic, IndoEuropean),
        (Hellenic, IndoEuropean),
        (IndoAryan, IndoEuropean),
        (Iranian, IndoEuropean),
        (Semitic, Afroasiatic),
        (Berber, Afroasiatic),
        (Cushitic, Afroasiatic),
        (Chadic, Afroasiatic),
        (Egyptian, Afroasiatic),
        (Bantu, NigerCongo),
        (Polynesian, Austronesian),
        (Sinitic, SinoTibetan),
    ];
    for lang in HUMAN_LANGUAGES {
        for (branch, parent) in parents {
            assert!(
                !lang.families.contains(&branch) || lang.families.contains(&parent),
                "{} is {branch} but not {parent}",
                lang.name
            );
        }
    }
}

#[test]
fn script_and_category_parse_errors_name_their_kind() {
    let err = "Moonrunes".parse::<ScriptName>().unwrap_err();
    assert_eq!(err.to_string(), r#"unrecognized script name: "Moonrunes""#);
    assert_eq!(err.input(), "Moonrunes");
    let err = "Telepathic".parse::<HumanLanguageCategory>().unwrap_err();
    assert_eq!(
        err.to_string(),
        r#"unrecognized category name: "Telepathic""#
    );
}

#[test]
fn lookup_errors_have_readable_messages() {
    let err = Greeting::text_for(
        Some(HumanLanguageName::Spanish),
        Some(DialectName::Mandarin),
        None,
    )
    .unwrap_err();
    assert_eq!(err.to_string(), "Mandarin is not recorded for Spanish");
}

#[test]
fn names_parse_without_accents_or_separators() {
    for input in [
        "Québécois French",               // exact
        "quebecois french",               // accents dropped
        "QUEBECOIS FRENCH",               // and uppercase
        "  Quebécois french ",            // mixed, with surrounding spaces
        "Que\u{301}be\u{301}cois French", // decomposed accents
        "QUÉBÉCOIS FRENCH",               // uppercase non-ASCII letters
        "Québécois-French",               // hyphen for the space
        "quebecois-french",               // hyphen, no accents
        "québécois_french",               // underscore for the space
        "QUEBECOIS_FRENCH",               // underscore, uppercase
        "Que\u{301}be\u{301}cois-French", // decomposed accents and hyphen
        "français québécois",             // endonym
        "francais-quebecois",             // endonym, no accents, hyphen
    ] {
        assert_eq!(input.parse(), Ok(DialectName::QuebecoisFrench), "{input:?}");
    }
    for input in ["Bokmål", "bokmal", "BOKMÅL", "Bokmal"] {
        assert_eq!(input.parse(), Ok(DialectName::Bokmal), "{input:?}");
    }
}

#[test]
fn accent_folding_does_not_loosen_matching_otherwise() {
    for input in [
        "quebec french",
        "quebecoisfrench",
        "quebocis_french", // misspelled: no fuzzy matching
        "quebecois.french",
        "bokma",
        "bokmål!",
    ] {
        assert!(
            input.parse::<DialectName>().is_err(),
            "{input:?} should not parse"
        );
    }
}

#[test]
fn endonyms_parse_like_display_names() {
    for (input, expected) in [
        ("日本語", HumanLanguageName::Japanese),
        ("Deutsch", HumanLanguageName::German),
        ("español", HumanLanguageName::Spanish),
        ("espanol", HumanLanguageName::Spanish), // accent-free fallback
        ("TIENG VIET", HumanLanguageName::Vietnamese), // accent-free, upper case
        ("  Čeština ", HumanLanguageName::Czech),
    ] {
        assert_eq!(input.parse(), Ok(expected), "{input:?}");
    }
    assert_eq!("関西弁".parse(), Ok(DialectName::KansaiJapanese));
    assert_eq!("hochdeutsch".parse(), Ok(DialectName::StandardGerman));
}

#[test]
fn endonym_is_separate_from_display() {
    assert_eq!(HumanLanguageName::Japanese.endonym(), Some("日本語"));
    assert_eq!(HumanLanguageName::Japanese.to_string(), "Japanese");
    assert_eq!(HumanLanguageName::English.endonym(), None);
    assert_eq!(HumanLanguageName::AmericanSignLanguage.endonym(), None);
}

#[test]
fn every_endonym_round_trips() {
    for &language in HumanLanguageName::ALL {
        if let Some(endonym) = language.endonym() {
            assert_eq!(endonym.parse(), Ok(language), "{endonym:?}");
        }
    }
    for &dialect in DialectName::ALL {
        if let Some(endonym) = dialect.endonym() {
            assert_eq!(endonym.parse(), Ok(dialect), "{endonym:?}");
        }
    }
}
