//! Hello-world greetings for `HumanLanguageName::Swahili`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/habari#Swahili>
//! - <https://en.wiktionary.org/wiki/dunia#Swahili>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Swahili => "Swahili" | "Kiswahili",
    dialects: [StandardSwahili => "Standard Swahili" | "Kiswahili sanifu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Swahili,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSwahili,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Habari, Dunia!",
        }],
    }],
};
