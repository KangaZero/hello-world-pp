//! Hello-world greetings for `HumanLanguageName::Kashubian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/wit%C3%B4j#Kashubian>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kashubian => "Kashubian" | "kaszëbsczi jãzëk",
    dialects: [StandardKashubian => "Standard Kashubian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kashubian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKashubian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Witôj, swiece!",
        }],
    }],
};
