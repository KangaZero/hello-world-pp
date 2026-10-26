//! Hello-world greetings for `HumanLanguageName::Latin`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/salve#Latin>
//! - <https://en.wiktionary.org/wiki/munde#Latin>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Dead, Liturgical},
    HumanLanguageFamily::{IndoEuropean, Italic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Latin => "Latin" | "Lingua Latina",
    dialects: [ClassicalLatin => "Classical Latin"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Latin,
    categories: &[Dead, Liturgical],
    families: &[IndoEuropean, Italic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ClassicalLatin,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salve, munde!",
        }],
    }],
};
