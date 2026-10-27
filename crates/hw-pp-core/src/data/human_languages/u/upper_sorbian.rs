//! Hello-world greetings for `HumanLanguageName::UpperSorbian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/dobry#Upper_Sorbian>
//! - <https://en.wiktionary.org/wiki/d%C5%BAe%C5%84#Upper_Sorbian>
//! - <https://en.wiktionary.org/wiki/sw%C4%9Bt#Upper_Sorbian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: UpperSorbian => "Upper Sorbian" | "hornjoserbšćina",
    dialects: [StandardUpperSorbian => "Standard Upper Sorbian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::UpperSorbian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardUpperSorbian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Dobry dźeń, swět!",
        }],
    }],
};
