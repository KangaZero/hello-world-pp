//! Hello-world greetings for `HumanLanguageName::Marathi`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A4%A8%E0%A4%AE%E0%A4%B8%E0%A5%8D%E0%A4%95%E0%A4%BE%E0%A4%B0#Marathi>
//! - <https://en.wiktionary.org/wiki/%E0%A4%9C%E0%A4%97#Marathi>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Marathi => "Marathi" | "मराठी",
    dialects: [StandardMarathi => "Standard Marathi"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Marathi,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Devanagari, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMarathi,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Devanagari],
                text: "नमस्कार, जग!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "namaskār, jag!",
            },
        ],
    }],
};
