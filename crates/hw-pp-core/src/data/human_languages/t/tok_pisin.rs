//! Hello-world greetings for `HumanLanguageName::TokPisin`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/graun#Tok_Pisin>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Creole, Natural},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: TokPisin => "Tok Pisin",
    dialects: [StandardTokPisin => "Standard Tok Pisin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::TokPisin,
    categories: &[Natural, Creole],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTokPisin,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halo, graun!",
        }],
    }],
};
