//! Hello-world greetings for `HumanLanguageName::Burmese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%80%99%E1%80%84%E1%80%BA%E1%80%B9%E1%80%82%E1%80%9C%E1%80%AC#Burmese>
//! - <https://en.wiktionary.org/wiki/%E1%80%95%E1%80%AB#Burmese>
//! - <https://en.wiktionary.org/wiki/%E1%80%80%E1%80%99%E1%80%B9%E1%80%98%E1%80%AC#Burmese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::SinoTibetan, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Burmese => "Burmese" | "မြန်မာဘာသာ",
    dialects: [StandardBurmese => "Standard Burmese"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Burmese,
    categories: &[Natural],
    families: &[SinoTibetan],
    scripts: &[ScriptName::Myanmar, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBurmese,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Myanmar],
                text: "မင်္ဂလာပါ ကမ္ဘာ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Mingalaba, kaba!",
            },
        ],
    }],
};
