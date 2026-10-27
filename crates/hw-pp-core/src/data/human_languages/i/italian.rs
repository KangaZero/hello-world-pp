//! Hello-world greetings for `HumanLanguageName::Italian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ciao#Italian>
//! - <https://en.wiktionary.org/wiki/mondo#Italian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Italian => "Italian" | "Italiano",
    dialects: [StandardItalian => "Standard Italian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Italian,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardItalian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ciao, mondo!",
        }],
    }],
};
