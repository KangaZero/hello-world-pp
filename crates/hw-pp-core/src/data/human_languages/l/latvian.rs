//! Hello-world greetings for `HumanLanguageName::Latvian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/sveika#Latvian>
//! - <https://en.wiktionary.org/wiki/pasaule#Latvian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Latvian => "Latvian" | "latviešu valoda",
    dialects: [StandardLatvian => "Standard Latvian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Latvian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLatvian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sveika, pasaule!",
        }],
    }],
};
