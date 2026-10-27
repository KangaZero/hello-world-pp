//! Hello-world greetings for `HumanLanguageName::Syriac`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%DC%AB%DC%A0%DC%A1%DC%90#Classical_Syriac>
//! - <https://en.wiktionary.org/wiki/%DC%A5%DC%A0%DC%A1%DC%90#Classical_Syriac>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Dead, Liturgical},
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Syriac => "Syriac" | "ܠܫܢܐ ܣܘܪܝܝܐ",
    dialects: [ClassicalSyriac => "Classical Syriac"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Syriac,
    categories: &[Dead, Liturgical],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Syriac, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ClassicalSyriac,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Syriac],
                text: "ܫܠܡܐ ܥܠܡܐ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "šlāmā ʿālmā!",
            },
        ],
    }],
};
