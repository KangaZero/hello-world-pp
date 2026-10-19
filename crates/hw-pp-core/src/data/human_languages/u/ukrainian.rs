//! Hello-world greetings for `HumanLanguageName::Ukrainian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Ukrainian => "Ukrainian" | "Українська",
    dialects: [StandardUkrainian => "Standard Ukrainian"],
    scripts: [Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Ukrainian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::StandardUkrainian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Cyrillic],
            text: "Привіт, світе!",
        }],
    }],
};
