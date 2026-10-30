//! Hello-world greetings for `HumanLanguageName::Neapolitan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/u%C3%A8#Neapolitan>
//! - <https://en.wiktionary.org/wiki/munno#Neapolitan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Neapolitan => "Neapolitan" | "napulitano",
    dialects: [StandardNeapolitan => "Standard Neapolitan"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Neapolitan,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardNeapolitan,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Uè, munno!",
        }],
    }],
};
