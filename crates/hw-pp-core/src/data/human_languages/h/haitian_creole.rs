//! Hello-world greetings for `HumanLanguageName::HaitianCreole`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bonjou#Haitian_Creole>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Creole, Natural},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: HaitianCreole => "Haitian Creole" | "kreyòl ayisyen",
    dialects: [StandardHaitianCreole => "Standard Haitian Creole"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::HaitianCreole,
    categories: &[Natural, Creole],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHaitianCreole,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bonjou, mond!",
        }],
    }],
};
