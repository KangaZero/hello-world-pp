//! Hello-world greetings for `HumanLanguageName::Asturian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hola#Asturian>
//! - <https://en.wiktionary.org/wiki/mundu#Asturian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Asturian => "Asturian" | "Asturianu",
    dialects: [StandardAsturian => "Standard Asturian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Asturian,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAsturian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "¡Hola, mundu!",
        }],
    }],
};
