//! Hello-world greetings for `HumanLanguageName::French`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bonjour#French>
//! - <https://en.wiktionary.org/wiki/le#French>
//! - <https://en.wiktionary.org/wiki/monde#French>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: French => "French" | "Français",
    dialects: [MetropolitanFrench => "Metropolitan French", QuebecoisFrench => "Québécois French" | "français québécois"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::French,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::MetropolitanFrench,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Bonjour le monde!",
            }],
        },
        Dialect {
            label: DialectName::QuebecoisFrench,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Bonjour le monde!",
            }],
        },
    ],
};
