//! Hello-world greetings for `HumanLanguageName::Hungarian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hell%C3%B3#Hungarian>
//! - <https://en.wiktionary.org/wiki/vil%C3%A1g#Hungarian>
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
    language: Hungarian => "Hungarian" | "Magyar",
    dialects: [StandardHungarian => "Standard Hungarian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hungarian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHungarian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Helló, világ!",
        }],
    }],
};
