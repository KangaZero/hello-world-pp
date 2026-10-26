//! Hello-world greetings for `HumanLanguageName::Zazaki`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/d%C4%B1nya#Zazaki>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Iranian},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Zazaki => "Zazaki",
    dialects: [StandardZazaki => "Standard Zazaki"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Zazaki,
    categories: &[Natural],
    families: &[IndoEuropean, Iranian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardZazaki,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sılam, dınya!",
        }],
    }],
};
