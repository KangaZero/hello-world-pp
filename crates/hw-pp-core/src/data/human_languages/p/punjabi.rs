//! Hello-world greetings for `HumanLanguageName::Punjabi`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A8%B8%E0%A8%A4%E0%A8%BF#Punjabi>
//! - <https://en.wiktionary.org/wiki/%E0%A8%B8%E0%A8%BC%E0%A9%8D%E0%A8%B0%E0%A9%80#Punjabi>
//! - <https://en.wiktionary.org/wiki/%E0%A8%85%E0%A8%95%E0%A8%BE%E0%A8%B2#Punjabi>
//! - <https://en.wiktionary.org/wiki/%E0%A8%A6%E0%A9%81%E0%A8%A8%E0%A9%80%E0%A8%86#Punjabi>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Punjabi => "Punjabi" | "ਪੰਜਾਬੀ",
    dialects: [EasternPunjabi => "Eastern Punjabi"],
    scripts: [Gurmukhi => "Gurmukhi", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Punjabi,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Gurmukhi, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::EasternPunjabi,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Gurmukhi],
                text: "ਸਤਿ ਸ਼੍ਰੀ ਅਕਾਲ, ਦੁਨੀਆ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "sat srī akāl, dunīā!",
            },
        ],
    }],
};
