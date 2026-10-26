//! Hello-world greetings for `HumanLanguageName::Ukrainian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%BF%D1%80%D0%B8%D0%B2%D1%96%D1%82#Ukrainian>
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B2%D1%96%D1%82%D0%B5#Ukrainian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Ukrainian => "Ukrainian" | "Українська",
    dialects: [StandardUkrainian => "Standard Ukrainian"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Ukrainian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardUkrainian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Привіт, світе!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Pryvít, svíte!",
            },
        ],
    }],
};
