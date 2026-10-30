//! Hello-world greetings for `HumanLanguageName::Croatian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/pozdrav#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Croatian => "Croatian" | "Hrvatski",
    dialects: [StandardCroatian => "Standard Croatian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Croatian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardCroatian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Pozdrav, svijete!",
        }],
    }],
};
