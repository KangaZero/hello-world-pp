//! Hello-world greetings for `HumanLanguageName::Albanian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/p%C3%ABrsh%C3%ABndetje#Albanian>
//! - <https://en.wiktionary.org/wiki/bot%C3%AB#Albanian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::IndoEuropean, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Albanian => "Albanian" | "Shqip",
    dialects: [StandardAlbanian => "Standard Albanian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Albanian,
    categories: &[Natural],
    families: &[IndoEuropean],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAlbanian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Përshëndetje, botë!",
        }],
    }],
};
