//! Hello-world greetings for `HumanLanguageName::Serbian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/zdravo#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/%D0%B7%D0%B4%D1%80%D0%B0%D0%B2%D0%BE#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/svete#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Serbian => "Serbian" | "српски",
    dialects: [StandardSerbian => "Standard Serbian"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Serbian,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSerbian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Здраво, свете!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Zdravo, svete!",
            },
        ],
    }],
};
