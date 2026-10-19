//! Hello-world greetings for `HumanLanguageName::Maori`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kia#M%C4%81ori>
//! - <https://en.wiktionary.org/wiki/ora#M%C4%81ori>
//! - <https://en.wiktionary.org/wiki/te#M%C4%81ori>
//! - <https://en.wiktionary.org/wiki/ao#M%C4%81ori>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Maori => "Māori" | "te reo Māori",
    dialects: [StandardMaori => "Standard Māori"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Maori,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMaori,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kia ora, te ao!",
        }],
    }],
};
