//! Hello-world greetings for `HumanLanguageName::Ossetian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%B4%D1%83%D0%BD%D0%B5#Ossetian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Ossetian => "Ossetian" | "ирон ӕвзаг",
    dialects: [IronOssetian => "Iron Ossetian" | "ирон"],
    scripts: [Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Ossetian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::IronOssetian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Cyrillic],
            text: "Салам, дуне!",
        }],
    }],
};
