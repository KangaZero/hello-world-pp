//! Hello-world greetings for `HumanLanguageName::Basque`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kaixo#Basque>
//! - <https://en.wiktionary.org/wiki/mundua#Basque>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Isolate, Natural},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Basque => "Basque" | "Euskara",
    dialects: [StandardBasque => "Standard Basque"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Basque,
    categories: &[Natural, Isolate],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBasque,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kaixo, mundua!",
        }],
    }],
};
