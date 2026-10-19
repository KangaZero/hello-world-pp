//! Hello-world greetings for `HumanLanguageName::Azerbaijani`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/salam#Azerbaijani>
//! - <https://en.wiktionary.org/wiki/d%C3%BCnya#Azerbaijani>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Azerbaijani => "Azerbaijani" | "Azərbaycan dili",
    dialects: [NorthAzerbaijani => "North Azerbaijani"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Azerbaijani,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::NorthAzerbaijani,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salam, dünya!",
        }],
    }],
};
