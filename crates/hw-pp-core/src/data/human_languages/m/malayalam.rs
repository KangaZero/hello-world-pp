//! Hello-world greetings for `HumanLanguageName::Malayalam`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%B4%A8%E0%B4%AE%E0%B4%B8%E0%B5%8D%E0%B4%95%E0%B4%BE%E0%B4%B0%E0%B4%82#Malayalam>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Dravidian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Malayalam => "Malayalam" | "മലയാളം",
    dialects: [StandardMalayalam => "Standard Malayalam"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Malayalam,
    categories: &[Natural],
    families: &[Dravidian],
    scripts: &[ScriptName::Malayalam, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMalayalam,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Malayalam],
                text: "നമസ്കാരം, ലോകമേ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "namaskāram, lōkamē!",
            },
        ],
    }],
};
