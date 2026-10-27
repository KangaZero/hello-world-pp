use hw_pp_core::data::human_languages::{
    HUMAN_LANGUAGES, HumanLanguageCategory, HumanLanguageName, ScriptName,
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
    pub greeting_count: usize,
    pub scripts: [ScriptStats; ScriptName::ALL.len()],
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
    pub fn language_count_without_script(script: Option<ScriptName>) -> usize {
        match script {
            Some(existing_script) => {
                HUMAN_LANGUAGES.len() - Self::language_count_with_script(existing_script)
            }
            None => HUMAN_LANGUAGES
                .iter()
                .filter(|language| language.scripts.is_empty())
                .count(),
        }
    }

    #[must_use]
    pub fn languages_without_script(script: Option<ScriptName>) -> Vec<HumanLanguageName> {
        match script {
            Some(existing_script) => HUMAN_LANGUAGES
                .iter()
                .filter(|language| !language.scripts.contains(&existing_script))
                .map(|lang| lang.name)
                .collect(),
            None => HUMAN_LANGUAGES
                .iter()
                .filter(|language| language.scripts.is_empty())
                .map(|lang| lang.name)
                .collect(),
        }
    }

    #[must_use]
    pub fn languages_without_category() -> Vec<HumanLanguageName> {
        HUMAN_LANGUAGES
            .iter()
            .filter(|language| language.categories.is_empty())
            .map(|lang| lang.name)
            .collect()
    }

    #[must_use]
    pub fn languages_without_dialect() -> Vec<HumanLanguageName> {
        HUMAN_LANGUAGES
            .iter()
            .filter(|language| language.dialects.is_empty())
            .map(|lang| lang.name)
            .collect()
    }

    #[must_use]
    pub fn languages_without_greeting() -> Vec<HumanLanguageName> {
        HUMAN_LANGUAGES
            .iter()
            .filter(|language| {
                language
                    .dialects
                    .iter()
                    .any(|dialect| dialect.greetings.is_empty())
            })
            .map(|lang| lang.name)
            .collect()
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
