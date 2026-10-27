//! Hello-world greetings for `HumanLanguageName::Basque`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kaixo#Basque>
//! - <https://en.wiktionary.org/wiki/mundua#Basque>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Isolate, Natural},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Basque => "Basque" | "Euskara",
    dialects: [StandardBasque => "Standard Basque"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Basque,
    categories: &[Natural, Isolate],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBasque,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kaixo, mundua!",
        }],
    }],
};
