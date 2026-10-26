//! Hello-world greetings for `HumanLanguageName::Bosnian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/zdravo#Serbo-Croatian>
//! - <https://en.wiktionary.org/wiki/%D0%B7%D0%B4%D1%80%D0%B0%D0%B2%D0%BE#Serbo-Croatian>
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
    language: Bosnian => "Bosnian" | "Bosanski",
    dialects: [StandardBosnian => "Standard Bosnian"],
    scripts: [Latin => "Latin", Cyrillic => "Cyrillic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bosnian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin, ScriptName::Cyrillic],
    dialects: &[Dialect {
        label: DialectName::StandardBosnian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Zdravo, svijete!",
            },
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Здраво, свијете!",
            },
        ],
    }],
};
