//! Hello-world greetings for `HumanLanguageName::Vietnamese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/xin#Vietnamese>
//! - <https://en.wiktionary.org/wiki/ch%C3%A0o#Vietnamese>
//! - <https://en.wiktionary.org/wiki/th%E1%BA%BF#Vietnamese>
//! - <https://en.wiktionary.org/wiki/gi%E1%BB%9Bi#Vietnamese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austroasiatic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Vietnamese => "Vietnamese" | "Tiếng Việt",
    dialects: [StandardVietnamese => "Standard Vietnamese"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Vietnamese,
    categories: &[Natural],
    families: &[Austroasiatic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardVietnamese,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Xin chào, thế giới!",
        }],
    }],
};
