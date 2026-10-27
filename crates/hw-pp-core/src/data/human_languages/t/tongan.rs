//! Hello-world greetings for `HumanLanguageName::Tongan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/lelei#Tongan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Austronesian, Polynesian},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tongan => "Tongan" | "lea faka-Tonga",
    dialects: [StandardTongan => "Standard Tongan"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tongan,
    categories: &[Natural],
    families: &[Austronesian, Polynesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTongan,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mālō e lelei, māmani!",
        }],
    }],
};
