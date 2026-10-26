//! Hello-world greetings for `HumanLanguageName::Bashkir`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D3%99%D0%BB%D3%99%D0%BC#Bashkir>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D0%BE%D0%BD%D1%8A%D1%8F#Bashkir>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Bashkir => "Bashkir" | "Башҡорт теле",
    dialects: [StandardBashkir => "Standard Bashkir"],
    scripts: [Cyrillic => "Cyrillic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bashkir,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBashkir,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Сәләм, донъя!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Säläm, dunya!",
            },
        ],
    }],
};
