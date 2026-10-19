//! Hello-world greetings for `HumanLanguageName::Portuguese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/ol%C3%A1#Portuguese>
//! - <https://en.wiktionary.org/wiki/mundo#Portuguese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Portuguese => "Portuguese" | "Português",
    dialects: [BrazilianPortuguese => "Brazilian Portuguese" | "português brasileiro", EuropeanPortuguese => "European Portuguese" | "português europeu"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Portuguese,
    categories: &[Natural],
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
