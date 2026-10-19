//! Hello-world greetings for `HumanLanguageName::Papiamento`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bon#Papiamentu>
//! - <https://en.wiktionary.org/wiki/dia#Papiamentu>
//! - <https://en.wiktionary.org/wiki/mundu#Papiamentu>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Creole, Natural},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Papiamento => "Papiamento" | "Papiamentu",
    dialects: [StandardPapiamento => "Standard Papiamento"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Papiamento,
    categories: &[Natural, Creole],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardPapiamento,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bon dia, mundu!",
        }],
    }],
};
