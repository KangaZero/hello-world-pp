//! Hello-world greetings for `HumanLanguageName::Udmurt`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D3%9F%D0%B5%D1%87%D0%B1%D1%83%D1%80#Udmurt>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D1%83%D0%BD%D0%BD%D0%B5#Udmurt>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Udmurt => "Udmurt" | "удмурт кыл",
    dialects: [StandardUdmurt => "Standard Udmurt"],
    scripts: [Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Udmurt,
    categories: &[Natural],
    scripts: &[ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::StandardUdmurt,
        greetings: &[Greeting {
            scripts: &[ScriptName::Cyrillic],
            text: "Ӟечбур, дунне!",
        }],
    }],
};
