//! Hello-world greetings for `HumanLanguageName::Breton`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/demat#Breton>
//! - <https://en.wiktionary.org/wiki/bed#Breton>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Breton => "Breton" | "Brezhoneg",
    dialects: [StandardBreton => "Standard Breton"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Breton,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBreton,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Demat, bed!",
        }],
    }],
};
