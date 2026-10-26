use hw_pp_core::data::programming_languages::{
    PROGRAMMING_LANGUAGES, ProgrammingLanguageCategory, ProgrammingLanguageFamily,
    ProgrammingLanguageName,
};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    clippy::struct_field_names,
    reason = "every field is a total; `_count` keeps `language_count` from reading as a list"
)]
pub struct ProgrammingLanguageStats {
    pub language_count: usize,
    pub dialect_count: usize,
    pub extension_count: usize,
    pub category_count: usize,
    pub family_count: usize,
}

impl ProgrammingLanguageStats {
    #[must_use]
    pub fn languages_without_extension() -> Vec<ProgrammingLanguageName> {
        PROGRAMMING_LANGUAGES
            .iter()
            .filter(|language| language.extensions.is_empty())
            .map(|lang| lang.name)
            .collect()
    }

    #[must_use]
    pub fn languages_without_family() -> Vec<ProgrammingLanguageName> {
        PROGRAMMING_LANGUAGES
            .iter()
            .filter(|language| language.families.is_empty())
            .map(|lang| lang.name)
            .collect()
    }

    #[must_use]
    // Note: Excludes `Whitespace` as the syntax is literally just spaces
    pub fn languages_without_syntax() -> Vec<ProgrammingLanguageName> {
        PROGRAMMING_LANGUAGES
            .iter()
            .filter(|language| {
                language.name != ProgrammingLanguageName::Whitespace
                    && language.versions.iter().any(|v| v.syntax.trim().is_empty())
            })
            .map(|lang| lang.name)
            .collect()
    }

    #[must_use]
    // Note: Excludes `Whitespace` as the syntax is literally just spaces
    pub fn languages_without_tokenized_syntax() -> Vec<ProgrammingLanguageName> {
        PROGRAMMING_LANGUAGES
            .iter()
            .filter(|language| {
                language.name != ProgrammingLanguageName::Whitespace
                    && language
                        .versions
                        .iter()
                        .any(|v| v.tokenized_syntax.len() <= 1)
            })
            .map(|lang| lang.name)
            .collect()
    }
}

/// Catalog totals, computed once on first use.
pub static PROGRAMMING_LANGUAGE_STATS: LazyLock<ProgrammingLanguageStats> =
    LazyLock::new(|| ProgrammingLanguageStats {
        language_count: PROGRAMMING_LANGUAGES.len(),
        dialect_count: PROGRAMMING_LANGUAGES
            .iter()
            .map(|language| language.versions.len())
            .sum(),
        category_count: ProgrammingLanguageCategory::all_as_str_by(None).len(),
        extension_count: PROGRAMMING_LANGUAGES
            .iter()
            .map(|language| language.extensions.len())
            .sum(),
        family_count: ProgrammingLanguageFamily::all_as_str_by(None).len(),
    });
