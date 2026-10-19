//! Hello-world greetings for `HumanLanguageName::Nepali`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A4%A8%E0%A4%AE%E0%A4%B8%E0%A5%8D%E0%A4%A4%E0%A5%87#Nepali>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Nepali => "Nepali" | "नेपाली",
    dialects: [StandardNepali => "Standard Nepali"],
    scripts: [Devanagari => "Devanagari", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Nepali,
    categories: &[Natural],
    scripts: &[ScriptName::Devanagari, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardNepali,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Devanagari],
                text: "नमस्ते, संसार!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "namaste, sansār!",
            },
        ],
    }],
};
