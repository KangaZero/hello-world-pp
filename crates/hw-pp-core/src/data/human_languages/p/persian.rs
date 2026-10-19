//! Hello-world greetings for `HumanLanguageName::Persian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D8%B3%D9%84%D8%A7%D9%85#Persian>
//! - <https://en.wiktionary.org/wiki/%D8%AF%D9%86%DB%8C%D8%A7#Persian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Persian => "Persian" | "فارسی",
    dialects: [IranianPersian => "Iranian Persian"],
    scripts: [PersoArabic => "Perso-Arabic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Persian,
    categories: &[Natural],
    scripts: &[ScriptName::PersoArabic],
    dialects: &[Dialect {
        label: DialectName::IranianPersian,
        greetings: &[Greeting {
            scripts: &[ScriptName::PersoArabic],
            text: "سلام، دنیا!",
        }],
    }],
};
