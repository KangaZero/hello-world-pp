//! Hello-world greetings for `HumanLanguageName::Tajik`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B0%D0%BB%D0%BE%D0%BC#Tajik>
//! - <https://en.wiktionary.org/wiki/%D2%B7%D0%B0%D2%B3%D0%BE%D0%BD#Tajik>
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
    language: Tajik => "Tajik" | "тоҷикӣ",
    dialects: [StandardTajik => "Standard Tajik"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tajik,
    categories: &[Natural],
    families: &[IndoEuropean, Iranian],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTajik,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Салом, ҷаҳон!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salom, jahon!",
            },
        ],
    }],
};
