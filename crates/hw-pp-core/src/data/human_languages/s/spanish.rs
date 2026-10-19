//! Hello-world greetings for `HumanLanguageName::Spanish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hola#Spanish>
//! - <https://en.wiktionary.org/wiki/mundo#Spanish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Spanish => "Spanish" | "Español",
    dialects: [CastilianSpanish => "Castilian Spanish" | "castellano", MexicanSpanish => "Mexican Spanish" | "español mexicano", RioplatenseSpanish => "Rioplatense Spanish" | "español rioplatense"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Spanish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::CastilianSpanish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "¡Hola, Mundo!",
            }],
        },
        Dialect {
            label: DialectName::MexicanSpanish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "¡Hola, Mundo!",
            }],
        },
        Dialect {
            label: DialectName::RioplatenseSpanish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "¡Hola, Mundo!",
            }],
        },
    ],
};
