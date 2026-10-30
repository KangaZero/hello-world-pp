//! Hello-world greetings for `HumanLanguageName::Tswana`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/dumela#Tswana>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Tswana => "Tswana" | "Setswana",
    dialects: [StandardTswana => "Standard Tswana"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tswana,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTswana,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Dumela, lefatshe!",
        }],
    }],
};
