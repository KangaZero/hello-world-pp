//! Hello-world greetings for `HumanLanguageName::Lingala`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mbote#Lingala>
//! - <https://en.wiktionary.org/wiki/mokili#Lingala>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Lingala => "Lingala",
    dialects: [StandardLingala => "Standard Lingala"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lingala,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLingala,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mbote, mokili!",
        }],
    }],
};
