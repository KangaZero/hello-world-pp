//! Hello-world greetings for `HumanLanguageName::Sicilian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/munnu#Sicilian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sicilian => "Sicilian" | "sicilianu",
    dialects: [StandardSicilian => "Standard Sicilian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sicilian,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSicilian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ciao, munnu!",
        }],
    }],
};
