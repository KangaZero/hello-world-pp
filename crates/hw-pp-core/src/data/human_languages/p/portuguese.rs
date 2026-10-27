//! Hello-world greetings for `HumanLanguageName::Portuguese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ol%C3%A1#Portuguese>
//! - <https://en.wiktionary.org/wiki/mundo#Portuguese>
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
    language: Portuguese => "Portuguese" | "Português",
    dialects: [BrazilianPortuguese => "Brazilian Portuguese" | "português brasileiro", EuropeanPortuguese => "European Portuguese" | "português europeu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Portuguese,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::BrazilianPortuguese,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Olá, Mundo!",
            }],
        },
        Dialect {
            label: DialectName::EuropeanPortuguese,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Olá, Mundo!",
            }],
        },
    ],
};
