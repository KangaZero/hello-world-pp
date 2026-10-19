//! Hello-world greetings for `HumanLanguageName::Galician`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ola#Galician>
//! - <https://en.wiktionary.org/wiki/mundo#Galician>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Galician => "Galician" | "galego",
    dialects: [StandardGalician => "Standard Galician"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Galician,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardGalician,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ola, mundo!",
        }],
    }],
};
