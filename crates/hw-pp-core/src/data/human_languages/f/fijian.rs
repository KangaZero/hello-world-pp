//! Hello-world greetings for `HumanLanguageName::Fijian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bula#Fijian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Fijian => "Fijian" | "vosa vakaviti",
    dialects: [StandardFijian => "Standard Fijian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Fijian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFijian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bula, vuravura!",
        }],
    }],
};
