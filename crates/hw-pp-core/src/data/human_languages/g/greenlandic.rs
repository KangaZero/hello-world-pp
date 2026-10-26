//! Hello-world greetings for `HumanLanguageName::Greenlandic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/aluu#Greenlandic>
//! - <https://en.wiktionary.org/wiki/nunarsuaq#Greenlandic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::EskimoAleut, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Greenlandic => "Greenlandic" | "kalaallisut",
    dialects: [WestGreenlandic => "West Greenlandic"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Greenlandic,
    categories: &[Natural],
    families: &[EskimoAleut],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::WestGreenlandic,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Aluu, nunarsuaq!",
        }],
    }],
};
