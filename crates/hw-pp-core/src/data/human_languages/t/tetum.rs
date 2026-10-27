//! Hello-world greetings for `HumanLanguageName::Tetum`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austronesian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tetum => "Tetum" | "Tetun",
    dialects: [StandardTetum => "Standard Tetum"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tetum,
    categories: &[Natural],
    families: &[Austronesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTetum,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Olá, mundu!",
        }],
    }],
};
