//! Hello-world greetings for `HumanLanguageName::Friulian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mond#Friulian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Friulian => "Friulian" | "furlan",
    dialects: [StandardFriulian => "Standard Friulian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Friulian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFriulian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mandi, mond!",
        }],
    }],
};
