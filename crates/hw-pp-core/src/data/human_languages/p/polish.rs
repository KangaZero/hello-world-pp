//! Hello-world greetings for `HumanLanguageName::Polish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/witaj#Polish>
//! - <https://en.wiktionary.org/wiki/%C5%9Bwiecie#Polish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Polish => "Polish" | "Polski",
    dialects: [StandardPolish => "Standard Polish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Polish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardPolish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Witaj, świecie!",
        }],
    }],
};
