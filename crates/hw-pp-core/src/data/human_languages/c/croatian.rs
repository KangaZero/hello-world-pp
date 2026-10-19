//! Hello-world greetings for `HumanLanguageName::Croatian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/pozdrav#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Croatian => "Croatian" | "Hrvatski",
    dialects: [StandardCroatian => "Standard Croatian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Croatian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardCroatian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Pozdrav, svijete!",
        }],
    }],
};
