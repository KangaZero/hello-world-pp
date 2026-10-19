//! Hello-world greetings for `HumanLanguageName::Uzbek`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/salom#Uzbek>
//! - <https://en.wiktionary.org/wiki/dunyo#Uzbek>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Uzbek => "Uzbek" | "oʻzbekcha",
    dialects: [StandardUzbek => "Standard Uzbek"],
    scripts: [Latin => "Latin", Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Uzbek,
    categories: &[Natural],
    scripts: &[ScriptName::Latin, ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::StandardUzbek,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salom, dunyo!",
            },
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Салом, дунё!",
            },
        ],
    }],
};
