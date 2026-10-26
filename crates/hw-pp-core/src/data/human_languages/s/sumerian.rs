//! Hello-world greetings for `HumanLanguageName::Sumerian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%F0%92%81%B2#Sumerian>
//! - <https://en.wiktionary.org/wiki/%F0%92%80%AD%F0%92%86%A0#Sumerian>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Extinct, Isolate},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sumerian => "Sumerian" | "𒅴𒂠",
    dialects: [Emegir => "Emegir" | "𒅴𒂠"],
    scripts: [Cuneiform => "Cuneiform", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sumerian,
    categories: &[Extinct, Isolate],
    scripts: &[ScriptName::Cuneiform, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::Emegir,
        greetings: &[
            // silim an-ki: "well-being, heaven-earth"; composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Cuneiform],
                text: "𒁲 𒀭𒆠",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "silim an-ki",
            },
        ],
    }],
};
