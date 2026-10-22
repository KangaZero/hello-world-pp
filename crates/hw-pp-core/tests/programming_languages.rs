use hw_pp_core::data::lookup::LookupError;
use hw_pp_core::data::programming_languages::{
    FileExtension, PROGRAMMING_LANGUAGES, ProgrammingLanguageCategory, ProgrammingLanguageDialect,
    ProgrammingLanguageError, ProgrammingLanguageFamily, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};

#[test]
fn every_language_has_a_version() {
    // Extensions are optional: many esoteric languages have no conventional one.
    for language in PROGRAMMING_LANGUAGES {
        assert!(
            !language.versions.is_empty(),
            "{} has no recorded versions",
            language.name
        );
    }
}

#[test]
fn programming_name_enums_round_trip_through_display_and_from_str() {
    for &name in ProgrammingLanguageName::ALL {
        assert_eq!(name.to_string().parse(), Ok(name));
    }
    for &version in ProgrammingLanguageDialect::ALL {
        assert_eq!(version.to_string().parse(), Ok(version));
    }
    for &category in ProgrammingLanguageCategory::ALL {
        assert_eq!(category.to_string().parse(), Ok(category));
    }
    for &family in ProgrammingLanguageFamily::ALL {
        assert_eq!(family.to_string().parse(), Ok(family));
    }
    assert_eq!("esolang".parse(), Ok(ProgrammingLanguageCategory::Esolang));
    assert_eq!("pl/i".parse(), Ok(ProgrammingLanguageFamily::PlI));
    assert_eq!(".net".parse(), Ok(ProgrammingLanguageFamily::DotNet));
    assert_eq!("c++".parse(), Ok(ProgrammingLanguageName::Cpp));
    assert_eq!("cobol".parse(), Ok(ProgrammingLanguageName::Cobol));
}

#[test]
fn every_programming_variant_is_used_in_the_data() {
    for name in ProgrammingLanguageName::ALL {
        assert!(
            PROGRAMMING_LANGUAGES.iter().any(|l| l.name == *name),
            "{name} has no entry"
        );
    }
    for version in ProgrammingLanguageDialect::ALL {
        assert!(
            PROGRAMMING_LANGUAGES
                .iter()
                .any(|l| l.versions.iter().any(|v| v.label == *version)),
            "{version} has no entry"
        );
    }
}

#[test]
fn extensions_round_trip_and_ignore_case() {
    for &ext in FileExtension::ALL {
        assert_eq!(ext.to_string().parse(), Ok(ext));
    }
    assert_eq!("R".parse(), Ok(FileExtension::R));
}

#[test]
fn programming_lookup_helpers_resolve_defaults_and_extensions() {
    assert_eq!(
        ProgrammingLanguageName::Cpp.default_version(),
        Ok(ProgrammingLanguageDialect::Cpp98)
    );
    assert_eq!(
        FileExtension::Rs.language(),
        Ok(ProgrammingLanguageName::Rust)
    );
}

#[test]
fn syntax_defaults_to_c99() {
    assert!(
        ProgrammingLanguageVersion::syntax_for(None, None, None)
            .expect("default resolves")
            .contains("printf(\"Hello, World!\\n\")")
    );
}

#[test]
fn syntax_infers_language_from_extension_or_version() {
    assert_eq!(
        ProgrammingLanguageVersion::syntax_for(None, None, Some(FileExtension::Py)),
        ProgrammingLanguageVersion::syntax_for(Some(ProgrammingLanguageName::Python), None, None)
    );
    assert_eq!(
        ProgrammingLanguageVersion::syntax_for(
            None,
            Some(ProgrammingLanguageDialect::Python2),
            None
        ),
        Ok(r#"print "Hello, World!""#)
    );
}

#[test]
fn syntax_rejects_extension_from_another_language() {
    assert_eq!(
        ProgrammingLanguageVersion::syntax_for(
            Some(ProgrammingLanguageName::Python),
            None,
            Some(FileExtension::Rs)
        ),
        Err(ProgrammingLanguageError::ExtensionMismatch {
            language: ProgrammingLanguageName::Python,
            extension: FileExtension::Rs,
        })
    );
    assert_eq!(
        ProgrammingLanguageVersion::syntax_for(
            None,
            Some(ProgrammingLanguageDialect::Cpp20),
            Some(FileExtension::Py)
        )
        .map_err(|err| err.to_string()),
        Err("py is not an extension of C++".to_owned())
    );
}

#[test]
fn syntax_rejects_version_from_another_language() {
    assert_eq!(
        ProgrammingLanguageVersion::syntax_for(
            Some(ProgrammingLanguageName::Rust),
            Some(ProgrammingLanguageDialect::Python3),
            None
        ),
        Err(ProgrammingLanguageError::Lookup(
            LookupError::UnknownVariant {
                language: ProgrammingLanguageName::Rust,
                variant: ProgrammingLanguageDialect::Python3,
            }
        ))
    );
}

#[test]
fn tokenized_syntax_len_matches_syntax() {
    for version in PROGRAMMING_LANGUAGES.iter().flat_map(|lang| lang.versions) {
        let tokens_len: usize = version
            .tokenized_syntax
            .iter()
            .map(|(_, text)| text.len())
            .sum();
        assert_eq!(
            tokens_len,
            version.syntax.len(),
            "{} tokens cover {tokens_len} bytes, but its syntax has {}",
            version.label,
            version.syntax.len()
        );
    }
}

#[test]
fn tokenized_syntax_rebuilds_every_snippet_exactly() {
    for version in PROGRAMMING_LANGUAGES.iter().flat_map(|lang| lang.versions) {
        let rebuilt: String = version
            .tokenized_syntax
            .iter()
            .map(|(_, text)| *text)
            .collect();
        assert_eq!(
            rebuilt, version.syntax,
            "{} tokens do not match its syntax",
            version.label
        );
        assert!(
            !version.tokenized_syntax.is_empty(),
            "{} has no tokens",
            version.label
        );
    }
}
