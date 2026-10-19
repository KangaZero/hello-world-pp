//! Hello-world greetings for `HumanLanguageName::Sanskrit`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A4%A8%E0%A4%AE%E0%A4%B8%E0%A5%8D%E0%A4%A4%E0%A5%87#Sanskrit>
//! - <https://en.wiktionary.org/wiki/%E0%A4%B5%E0%A4%BF%E0%A4%B6%E0%A5%8D%E0%A4%B5#Sanskrit>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Liturgical, Natural},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sanskrit => "Sanskrit" | "संस्कृतम्",
    dialects: [ClassicalSanskrit => "Classical Sanskrit"],
    scripts: [Devanagari => "Devanagari"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sanskrit,
    categories: &[Liturgical, Natural],
    scripts: &[ScriptName::Devanagari],
    dialects: &[Dialect {
        label: DialectName::ClassicalSanskrit,
        greetings: &[Greeting {
            scripts: &[ScriptName::Devanagari],
            text: "नमस्ते विश्व!",
        }],
    }],
};
