//! Hello-world greetings for `HumanLanguageName::Zulu`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/sawubona#Zulu>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Zulu => "Zulu" | "isiZulu",
    dialects: [StandardZulu => "Standard Zulu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Zulu,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardZulu,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sawubona, Mhlaba!",
        }],
    }],
};
