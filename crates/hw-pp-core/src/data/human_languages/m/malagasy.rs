//! Hello-world greetings for `HumanLanguageName::Malagasy`.
//!
//! Imported from the Hello World Collection (`files/Human.htm` at `8e4c095`),
//! MIT licensed; see `human_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/manao#Malagasy>
//! - <https://en.wiktionary.org/wiki/ahoana#Malagasy>
//! - <https://en.wiktionary.org/wiki/tany#Malagasy>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Malagasy => "Malagasy",
    dialects: [StandardMalagasy => "Standard Malagasy"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Malagasy,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMalagasy,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Manao ahoana ry tany!",
        }],
    }],
};
