//! Hello-world greetings for `HumanLanguageName::Kannada`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%B2%A8%E0%B2%AE%E0%B2%B8%E0%B3%8D%E0%B2%95%E0%B2%BE%E0%B2%B0#Kannada>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Dravidian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kannada => "Kannada" | "ಕನ್ನಡ",
    dialects: [StandardKannada => "Standard Kannada"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kannada,
    categories: &[Natural],
    families: &[Dravidian],
    scripts: &[ScriptName::Kannada, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKannada,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Kannada],
                text: "ನಮಸ್ಕಾರ, ವಿಶ್ವ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Namaskāra, viśva!",
            },
        ],
    }],
};
