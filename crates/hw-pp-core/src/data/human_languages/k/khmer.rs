//! Hello-world greetings for `HumanLanguageName::Khmer`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%9E%9F%E1%9E%BD%E1%9E%9F%E1%9F%92%E1%9E%8F%E1%9E%B8#Khmer>
//! - <https://en.wiktionary.org/wiki/%E1%9E%96%E1%9E%B7%E1%9E%97%E1%9E%96%E1%9E%9B%E1%9F%84%E1%9E%80#Khmer>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austroasiatic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Khmer => "Khmer" | "ភាសាខ្មែរ",
    dialects: [StandardKhmer => "Standard Khmer"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Khmer,
    categories: &[Natural],
    families: &[Austroasiatic],
    scripts: &[ScriptName::Khmer, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKhmer,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Khmer],
                text: "សួស្តី ពិភពលោក!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Suəsdəy piphup look!",
            },
        ],
    }],
};
