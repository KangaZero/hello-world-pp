//! Hello-world greetings for `HumanLanguageName::Russian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Russian => "Russian",
    dialects: [StandardRussian => "Standard Russian"],
    scripts: [Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Russian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::StandardRussian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Cyrillic],
            text: "Привет, мир!",
        }],
    }],
};
