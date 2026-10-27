//! Hello-world greetings for `HumanLanguageName::Volapuk`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/glidis#Volap%C3%BCk>
//! - <https://en.wiktionary.org/wiki/vol#Volap%C3%BCk>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Volapuk => "Volapük",
    dialects: [StandardVolapuk => "Standard Volapük"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Volapuk,
    categories: &[Constructed, Auxiliary],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardVolapuk,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Glidis, vol!",
        }],
    }],
};
