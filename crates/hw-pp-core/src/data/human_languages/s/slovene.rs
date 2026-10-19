//! Hello-world greetings for `HumanLanguageName::Slovene`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/svet#Slovene>
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
    language: Slovene => "Slovene" | "slovenščina",
    dialects: [StandardSlovene => "Standard Slovene"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Slovene,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSlovene,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Pozdravljen, svet!",
        }],
    }],
};
