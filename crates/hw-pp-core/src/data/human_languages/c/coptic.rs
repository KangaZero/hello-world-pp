//! Hello-world greetings for `HumanLanguageName::Coptic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E2%B2%AD%E2%B2%89%E2%B2%A3%E2%B2%89#Coptic>
//! - <https://en.wiktionary.org/wiki/%E2%B2%95%E2%B2%9F%E2%B2%A5%E2%B2%99%E2%B2%9F%E2%B2%A5#Coptic>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Dead, Liturgical},
    HumanLanguageFamily::{Afroasiatic, Egyptian},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Coptic => "Coptic",
    dialects: [BohairicCoptic => "Bohairic Coptic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Coptic,
    categories: &[Dead, Liturgical],
    families: &[Afroasiatic, Egyptian],
    scripts: &[ScriptName::Coptic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::BohairicCoptic,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Coptic],
                text: "ⲭⲉⲣⲉ, ⲕⲟⲥⲙⲟⲥ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "khere, kosmos!",
            },
        ],
    }],
};
