use hw_pp_core::data::human_languages::{
    HUMAN_LANGUAGES, HumanLanguage, HumanLanguageCategory, HumanLanguageFamily, HumanLanguageName,
    ScriptName,
};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptStats {
    pub name: ScriptName,
    pub language_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumanLanguageStats {
    pub language_count: usize,
    pub dialect_count: usize,
    pub category_count: usize,
    pub family_count: usize,
    pub greeting_count: usize,
    pub scripts: [ScriptStats; ScriptName::ALL.len()],
}

/// Categories whose languages have no family by definition, so an empty `families` is not a gap.
const FAMILYLESS_CATEGORIES: [HumanLanguageCategory; 4] = [
    HumanLanguageCategory::Constructed,
    HumanLanguageCategory::Isolate,
    HumanLanguageCategory::Creole,
    HumanLanguageCategory::Pidgin,
];

/// Names of the languages matching `predicate`, in catalog order.
fn languages_where(predicate: impl Fn(&HumanLanguage) -> bool) -> Vec<HumanLanguageName> {
    HUMAN_LANGUAGES
        .iter()
        .filter(|language| predicate(language))
        .map(|language| language.name)
        .collect()
}

impl HumanLanguageStats {
    #[must_use]
    pub fn language_count_with_script(script: ScriptName) -> usize {
        HUMAN_LANGUAGES
            .iter()
            .filter(|language| language.scripts.contains(&script))
            .count()
    }

    #[must_use]
    pub fn languages_without_script(script: Option<ScriptName>) -> Vec<HumanLanguageName> {
        match script {
            Some(existing_script) => {
                languages_where(|language| !language.scripts.contains(&existing_script))
            }
            None => languages_where(|language| language.scripts.is_empty()),
        }
    }

    #[must_use]
    pub fn languages_without_category() -> Vec<HumanLanguageName> {
        languages_where(|language| language.categories.is_empty())
    }

    /// Languages with no family recorded, excluding [`FAMILYLESS_CATEGORIES`].
    #[must_use]
    pub fn languages_without_family() -> Vec<HumanLanguageName> {
        languages_where(|language| {
            language.families.is_empty()
                && !language
                    .categories
                    .iter()
                    .any(|category| FAMILYLESS_CATEGORIES.contains(category))
        })
    }

    #[must_use]
    pub fn languages_without_dialect() -> Vec<HumanLanguageName> {
        languages_where(|language| language.dialects.is_empty())
    }

    #[must_use]
    pub fn languages_without_greeting() -> Vec<HumanLanguageName> {
        languages_where(|language| {
            language
                .dialects
                .iter()
                .any(|dialect| dialect.greetings.is_empty())
        })
    }
}

/// Catalog totals, computed once on first use.
pub static HUMAN_LANGUAGE_STATS: LazyLock<HumanLanguageStats> =
    LazyLock::new(|| HumanLanguageStats {
        language_count: HUMAN_LANGUAGES.len(),
        dialect_count: HUMAN_LANGUAGES
            .iter()
            .map(|language| language.dialects.len())
            .sum(),
        category_count: HumanLanguageCategory::all_by(None).count(),
        family_count: HumanLanguageFamily::all_by(None).count(),
        greeting_count: HUMAN_LANGUAGES
            .iter()
            .flat_map(|languages| languages.dialects.iter())
            .map(|dialect| dialect.greetings.len())
            .sum(),
        scripts: std::array::from_fn(|index| {
            let name = ScriptName::ALL[index];
            ScriptStats {
                name,
                language_count: HumanLanguageStats::language_count_with_script(name),
            }
        }),
    });
