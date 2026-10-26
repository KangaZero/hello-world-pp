//! Hello-world greetings for `HumanLanguageName::Tigrinya`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%88%B0%E1%88%8B%E1%88%9D#Tigrinya>
//! - <https://en.wiktionary.org/wiki/%E1%8B%93%E1%88%88%E1%88%9D#Tigrinya>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tigrinya => "Tigrinya" | "ትግርኛ",
    dialects: [StandardTigrinya => "Standard Tigrinya"],
    scripts: [Ethiopic => "Ethiopic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tigrinya,
    categories: &[Natural],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Ethiopic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTigrinya,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Ethiopic],
                text: "ሰላም ዓለም!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "sälam ʿaläm!",
            },
        ],
    }],
};
