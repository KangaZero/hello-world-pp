//! Hello-world greetings for `HumanLanguageName::Maltese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bon%C4%A1u#Maltese>
//! - <https://en.wiktionary.org/wiki/dinja#Maltese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Maltese => "Maltese" | "Malti",
    dialects: [StandardMaltese => "Standard Maltese"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Maltese,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMaltese,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bonġu, dinja!",
        }],
    }],
};
