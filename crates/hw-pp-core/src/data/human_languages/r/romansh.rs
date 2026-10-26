//! Hello-world greetings for `HumanLanguageName::Romansh`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/allegra#Romansh>
//! - <https://en.wiktionary.org/wiki/mund#Romansh>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Romansh => "Romansh" | "rumantsch",
    dialects: [RumantschGrischun => "Rumantsch Grischun"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Romansh,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::RumantschGrischun,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Allegra, mund!",
        }],
    }],
};
