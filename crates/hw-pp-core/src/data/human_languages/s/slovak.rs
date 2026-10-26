//! Hello-world greetings for `HumanLanguageName::Slovak`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ahoj#Slovak>
//! - <https://en.wiktionary.org/wiki/svet#Slovak>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Slovak => "Slovak" | "slovenčina",
    dialects: [StandardSlovak => "Standard Slovak"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Slovak,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSlovak,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ahoj, svet!",
        }],
    }],
};
