//! Hello-world greetings for `HumanLanguageName::Italian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ciao#Italian>
//! - <https://en.wiktionary.org/wiki/mondo#Italian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Italian => "Italian" | "Italiano",
    dialects: [StandardItalian => "Standard Italian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Italian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardItalian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ciao, mondo!",
        }],
    }],
};
