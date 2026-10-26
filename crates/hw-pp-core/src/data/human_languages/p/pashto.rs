//! Hello-world greetings for `HumanLanguageName::Pashto`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D9%86%DA%93%DB%8D#Pashto>
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
    language: Pashto => "Pashto" | "پښتو",
    dialects: [StandardPashto => "Standard Pashto"],
    scripts: [PersoArabic => "Perso-Arabic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Pashto,
    categories: &[Natural],
    families: &[IndoEuropean, Iranian],
    scripts: &[ScriptName::PersoArabic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardPashto,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::PersoArabic],
                text: "سلام، نړۍ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "salām, naṛəy!",
            },
        ],
    }],
};
