//! Hello-world greetings for `HumanLanguageName::Shona`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Shona => "Shona" | "chiShona",
    dialects: [StandardShona => "Standard Shona"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Shona,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardShona,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mhoro, nyika!",
        }],
    }],
};
