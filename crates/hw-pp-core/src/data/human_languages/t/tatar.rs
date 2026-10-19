//! Hello-world greetings for `HumanLanguageName::Tatar`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D3%99%D0%BB%D0%B0%D0%BC#Tatar>
//! - <https://en.wiktionary.org/wiki/d%C3%B6nya#Tatar>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D3%A9%D0%BD%D1%8C%D1%8F#Tatar>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tatar => "Tatar" | "татар теле",
    dialects: [StandardTatar => "Standard Tatar"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tatar,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTatar,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Сәлам, дөнья!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Sälam, dönya!",
            },
        ],
    }],
};
