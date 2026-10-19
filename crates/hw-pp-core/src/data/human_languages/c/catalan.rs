//! Hello-world greetings for `HumanLanguageName::Catalan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hola#Catalan>
//! - <https://en.wiktionary.org/wiki/m%C3%B3n#Catalan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Catalan => "Catalan" | "Català",
    dialects: [StandardCatalan => "Standard Catalan"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Catalan,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardCatalan,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hola, món!",
        }],
    }],
};
