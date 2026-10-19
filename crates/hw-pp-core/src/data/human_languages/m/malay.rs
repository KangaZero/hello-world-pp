//! Hello-world greetings for `HumanLanguageName::Malay`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/helo#Malay>
//! - <https://en.wiktionary.org/wiki/dunia#Malay>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Malay => "Malay" | "Bahasa Melayu",
    dialects: [StandardMalay => "Standard Malay"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Malay,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMalay,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Helo, dunia!",
        }],
    }],
};
