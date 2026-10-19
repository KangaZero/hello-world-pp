//! Hello-world greetings for `HumanLanguageName::Zulu`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/sawubona#Zulu>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Zulu => "Zulu" | "isiZulu",
    dialects: [StandardZulu => "Standard Zulu"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Zulu,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardZulu,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sawubona, Mhlaba!",
        }],
    }],
};
