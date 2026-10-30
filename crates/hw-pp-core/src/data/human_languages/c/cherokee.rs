//! Hello-world greetings for `HumanLanguageName::Cherokee`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%8E%A3%E1%8F%8F%E1%8F%B2#Cherokee>
//! - <https://en.wiktionary.org/wiki/%E1%8E%A1%E1%8E%B6%E1%8E%AF#Cherokee>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Iroquoian, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Cherokee => "Cherokee" | "ᏣᎳᎩ",
    dialects: [WesternCherokee => "Western Cherokee"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Cherokee,
    categories: &[Natural],
    families: &[Iroquoian],
    scripts: &[ScriptName::Cherokee, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::WesternCherokee,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cherokee],
                text: "ᎣᏏᏲ ᎡᎶᎯ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Osiyo, elohi!",
            },
        ],
    }],
};
