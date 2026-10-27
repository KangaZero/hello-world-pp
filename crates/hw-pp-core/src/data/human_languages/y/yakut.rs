//! Hello-world greetings for `HumanLanguageName::Yakut`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%B4%D0%BE%D1%80%D0%BE%D0%BE%D0%B1%D0%BE#Yakut>
//! - <https://en.wiktionary.org/wiki/%D0%B0%D0%B0%D0%BD#Yakut>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D0%BE%D0%B9%D0%B4%D1%83#Yakut>
//! - <https://en.wiktionary.org/wiki/%D0%B0%D0%B0%D0%BD_%D0%B4%D0%BE%D0%B9%D0%B4%D1%83#Yakut>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Turkic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Yakut => "Yakut" | "саха тыла",
    dialects: [StandardYakut => "Standard Yakut"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Yakut,
    categories: &[Natural],
    families: &[Turkic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardYakut,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Дорообо, аан дойду!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Doroobo, aan doydu!",
            },
        ],
    }],
};
