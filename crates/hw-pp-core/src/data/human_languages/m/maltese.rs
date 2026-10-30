//! Hello-world greetings for `HumanLanguageName::Maltese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%C4%A7elow#Maltese>
//! - <https://en.wiktionary.org/wiki/dinja#Maltese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Maltese => "Maltese" | "Malti",
    dialects: [StandardMaltese => "Standard Maltese"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Maltese,
    categories: &[Natural],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMaltese,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ħelow, dinja!",
        }],
    }],
};
