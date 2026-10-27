//! Hello-world greetings for `HumanLanguageName::Polish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/witaj#Polish>
//! - <https://en.wiktionary.org/wiki/%C5%9Bwiecie#Polish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Polish => "Polish" | "Polski",
    dialects: [StandardPolish => "Standard Polish"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Polish,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardPolish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Witaj, świecie!",
        }],
    }],
};
