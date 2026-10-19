//! Hello-world greetings for `HumanLanguageName::Hmong`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/nyob#White_Hmong>
//! - <https://en.wiktionary.org/wiki/zoo#White_Hmong>
//! - <https://en.wiktionary.org/wiki/teb#White_Hmong>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hmong => "Hmong" | "Hmoob",
    dialects: [WhiteHmong => "White Hmong" | "Hmoob Dawb"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hmong,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::WhiteHmong,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Nyob zoo, ntiaj teb!",
        }],
    }],
};
