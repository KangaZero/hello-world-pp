//! Hello-world greetings for `HumanLanguageName::Hawaiian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/aloha#Hawaiian>
//! - <https://en.wiktionary.org/wiki/honua#Hawaiian>
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
    language: Hawaiian => "Hawaiian" | "ʻŌlelo Hawaiʻi",
    dialects: [StandardHawaiian => "Standard Hawaiian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hawaiian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHawaiian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Aloha, honua!",
        }],
    }],
};
