//! Hello-world greetings for `HumanLanguageName::Interlingua`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mundo#Interlingua>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Interlingua => "Interlingua",
    dialects: [StandardInterlingua => "Standard Interlingua"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Interlingua,
    categories: &[Constructed, Auxiliary],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardInterlingua,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salute, mundo!",
        }],
    }],
};
