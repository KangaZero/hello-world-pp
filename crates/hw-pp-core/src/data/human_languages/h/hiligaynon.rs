//! Hello-world greetings for `HumanLanguageName::Hiligaynon`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kalibutan#Hiligaynon>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austronesian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hiligaynon => "Hiligaynon",
    dialects: [StandardHiligaynon => "Standard Hiligaynon"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hiligaynon,
    categories: &[Natural],
    families: &[Austronesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHiligaynon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kamusta, kalibutan!",
        }],
    }],
};
