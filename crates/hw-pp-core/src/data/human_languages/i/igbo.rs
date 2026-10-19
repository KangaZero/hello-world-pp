//! Hello-world greetings for `HumanLanguageName::Igbo`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Igbo => "Igbo",
    dialects: [StandardIgbo => "Standard Igbo"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Igbo,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIgbo,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ndewo, ụwa!",
        }],
    }],
};
