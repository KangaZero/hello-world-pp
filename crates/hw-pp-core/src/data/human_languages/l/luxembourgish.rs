//! Hello-world greetings for `HumanLanguageName::Luxembourgish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/Moien#Luxembourgish>
//! - <https://en.wiktionary.org/wiki/Welt#Luxembourgish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Luxembourgish => "Luxembourgish" | "Lëtzebuergesch",
    dialects: [StandardLuxembourgish => "Standard Luxembourgish"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Luxembourgish,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLuxembourgish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Moien, Welt!",
        }],
    }],
};
