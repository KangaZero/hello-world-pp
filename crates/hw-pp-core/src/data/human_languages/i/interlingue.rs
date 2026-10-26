//! Hello-world greetings for `HumanLanguageName::Interlingue`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Interlingue>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Interlingue => "Interlingue",
    dialects: [StandardInterlingue => "Standard Interlingue"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Interlingue,
    categories: &[Constructed, Auxiliary],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardInterlingue,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salute, munde!",
        }],
    }],
};
