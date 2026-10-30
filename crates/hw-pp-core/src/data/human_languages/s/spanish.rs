//! Hello-world greetings for `HumanLanguageName::Spanish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hola#Spanish>
//! - <https://en.wiktionary.org/wiki/mundo#Spanish>
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

use hw_pp_macros::language_names;

language_names! {
    language: Spanish => "Spanish" | "Español",
    dialects: [CastilianSpanish => "Castilian Spanish" | "castellano", MexicanSpanish => "Mexican Spanish" | "español mexicano", RioplatenseSpanish => "Rioplatense Spanish" | "español rioplatense"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Spanish,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
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
