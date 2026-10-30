//! Hello-world greetings for `HumanLanguageName::Samoan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/talofa#Samoan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Austronesian, Polynesian},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Samoan => "Samoan" | "Gagana Sāmoa",
    dialects: [StandardSamoan => "Standard Samoan"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Samoan,
    categories: &[Natural],
    families: &[Austronesian, Polynesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSamoan,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Talofa, lalolagi!",
        }],
    }],
};
