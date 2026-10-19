//! Hello-world greetings for `HumanLanguageName::Kyrgyz`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B0%D0%BB%D0%B0%D0%BC#Kyrgyz>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D2%AF%D0%B9%D0%BD%D3%A9#Kyrgyz>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kyrgyz => "Kyrgyz" | "кыргыз тили",
    dialects: [StandardKyrgyz => "Standard Kyrgyz"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kyrgyz,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKyrgyz,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Салам, дүйнө!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salam, düynö!",
            },
        ],
    }],
};
