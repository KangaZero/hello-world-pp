use hw_pp_core::data::programming_languages::{
    PROGRAMMING_LANGUAGES, ProgrammingLanguage, ProgrammingLanguageCategory,
    ProgrammingLanguageFamily, ProgrammingLanguageName, ProgrammingParadigm,
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
    pub paradigm_count: usize,
}

/// Names of the languages matching `predicate`, in catalog order.
fn languages_where(
    predicate: impl Fn(&ProgrammingLanguage) -> bool,
) -> Vec<ProgrammingLanguageName> {
    PROGRAMMING_LANGUAGES
        .iter()
        .filter(|language| predicate(language))
        .map(|language| language.name)
        .collect()
}

impl ProgrammingLanguageStats {
    #[must_use]
    pub fn languages_without_extension() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| language.extensions.is_empty())
    }

    #[must_use]
    pub fn languages_without_category() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| language.categories.is_empty())
    }

    #[must_use]
    pub fn languages_without_family() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| language.families.is_empty())
    }

    #[must_use]
    pub fn languages_without_paradigm() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| language.paradigms.is_empty())
    }

    #[must_use]
    // Note: Excludes `Whitespace` as the syntax is literally just spaces
    pub fn languages_without_syntax() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| {
            language.name != ProgrammingLanguageName::Whitespace
                && language.versions.iter().any(|v| v.syntax.trim().is_empty())
        })
    }

    #[must_use]
    // Note: Excludes `Whitespace` as the syntax is literally just spaces
    pub fn languages_without_tokenized_syntax() -> Vec<ProgrammingLanguageName> {
        languages_where(|language| {
            language.name != ProgrammingLanguageName::Whitespace
                && language
                    .versions
                    .iter()
                    .any(|v| v.tokenized_syntax.len() <= 1)
        })
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
        category_count: ProgrammingLanguageCategory::all_by(None).count(),
        extension_count: PROGRAMMING_LANGUAGES
            .iter()
            .map(|language| language.extensions.len())
            .sum(),
        family_count: ProgrammingLanguageFamily::all_by(None).count(),
        paradigm_count: ProgrammingParadigm::all_by(None).count(),
    });
