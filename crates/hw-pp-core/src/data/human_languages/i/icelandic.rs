//! Hello-world greetings for `HumanLanguageName::Icelandic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hall%C3%B3#Icelandic>
//! - <https://en.wiktionary.org/wiki/heimur#Icelandic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Icelandic => "Icelandic" | "íslenska",
    dialects: [StandardIcelandic => "Standard Icelandic"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Icelandic,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIcelandic,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halló, heimur!",
        }],
    }],
};
