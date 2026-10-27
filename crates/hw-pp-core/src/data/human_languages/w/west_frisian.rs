//! Hello-world greetings for `HumanLanguageName::WestFrisian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hallo#West_Frisian>
//! - <https://en.wiktionary.org/wiki/wr%C3%A2ld#West_Frisian>
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
    language: WestFrisian => "West Frisian" | "Frysk",
    dialects: [StandardWestFrisian => "Standard West Frisian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::WestFrisian,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardWestFrisian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hallo, wrâld!",
        }],
    }],
};
