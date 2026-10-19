//! Hello-world greetings for `HumanLanguageName::Luxembourgish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/Moien#Luxembourgish>
//! - <https://en.wiktionary.org/wiki/Welt#Luxembourgish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Luxembourgish => "Luxembourgish" | "Lëtzebuergesch",
    dialects: [StandardLuxembourgish => "Standard Luxembourgish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Luxembourgish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLuxembourgish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Moien, Welt!",
        }],
    }],
};
