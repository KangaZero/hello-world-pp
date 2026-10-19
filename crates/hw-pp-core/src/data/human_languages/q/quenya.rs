//! Hello-world greetings for `HumanLanguageName::Quenya`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Quenya>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Constructed, Fictional},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Quenya => "Quenya",
    dialects: [ExilicQuenya => "Exilic Quenya"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Quenya,
    categories: &[Constructed, Fictional],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ExilicQuenya,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Aiya, Ambar!",
        }],
    }],
};
