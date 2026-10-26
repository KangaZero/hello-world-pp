//! Hello-world greetings for `HumanLanguageName::Russian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%BF%D1%80%D0%B8%D0%B2%D0%B5%D1%82#Russian>
//! - <https://en.wiktionary.org/wiki/%D0%BC%D0%B8%D1%80#Russian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Russian => "Russian" | "Русский",
    dialects: [StandardRussian => "Standard Russian"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Russian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardRussian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Привет, мир!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Privét, mir!",
            },
        ],
    }],
};
