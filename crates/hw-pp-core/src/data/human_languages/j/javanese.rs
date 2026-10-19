//! Hello-world greetings for `HumanLanguageName::Javanese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%EA%A6%B2%EA%A6%AD#Javanese>
//! - <https://en.wiktionary.org/wiki/halo#Javanese>
//! - <https://en.wiktionary.org/wiki/%EA%A6%A2%EA%A6%BA%EA%A6%B4%EA%A6%9A#Javanese>
//! - <https://en.wiktionary.org/wiki/donya#Javanese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Javanese => "Javanese" | "basa Jawa",
    dialects: [StandardJavanese => "Standard Javanese"],
    scripts: [Javanese => "Javanese", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Javanese,
    categories: &[Natural],
    scripts: &[ScriptName::Javanese, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardJavanese,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Javanese],
                text: "ꦲꦭꦺꦴ꧈ ꦢꦺꦴꦚ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Halo, donya!",
            },
        ],
    }],
};
