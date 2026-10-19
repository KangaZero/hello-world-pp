//! Hello-world greetings for `HumanLanguageName::Hausa`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/sannu#Hausa>
//! - <https://en.wiktionary.org/wiki/duniya#Hausa>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hausa => "Hausa",
    dialects: [StandardHausa => "Standard Hausa"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hausa,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHausa,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sannu, duniya!",
        }],
    }],
};
