//! Hello-world greetings for `HumanLanguageName::Sotho`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/dumela#Sotho>
//! - <https://en.wiktionary.org/wiki/lefatshe#Sotho>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sotho => "Sotho" | "Sesotho",
    dialects: [StandardSotho => "Standard Sotho"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sotho,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSotho,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Dumela, lefatshe!",
        }],
    }],
};
