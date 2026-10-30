//! Hello-world greetings for `HumanLanguageName::Igbo`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::NigerCongo, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Igbo => "Igbo",
    dialects: [StandardIgbo => "Standard Igbo"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Igbo,
    categories: &[Natural],
    families: &[NigerCongo],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIgbo,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ndewo, ụwa!",
        }],
    }],
};
