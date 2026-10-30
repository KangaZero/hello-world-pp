//! Hello-world greetings for `HumanLanguageName::Ido`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hola#Ido>
//! - <https://en.wiktionary.org/wiki/mondo#Ido>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Ido => "Ido",
    dialects: [StandardIdo => "Standard Ido"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Ido,
    categories: &[Constructed, Auxiliary],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIdo,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hola, mondo!",
        }],
    }],
};
