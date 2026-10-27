//! Hello-world greetings for `HumanLanguageName::Lezgian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%B4%D1%83%D1%8C%D0%BD%D1%8C%D1%8F#Lezgi>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::NortheastCaucasian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Lezgian => "Lezgian" | "лезги чӀал",
    dialects: [StandardLezgian => "Standard Lezgian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lezgian,
    categories: &[Natural],
    families: &[NortheastCaucasian],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLezgian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Салам, дуьнья!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salam, dün’ä!",
            },
        ],
    }],
};
