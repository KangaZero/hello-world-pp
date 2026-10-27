//! Hello-world greetings for `HumanLanguageName::Yoruba`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%BA%B8#Yoruba>
//! - <https://en.wiktionary.org/wiki/n#Yoruba>
//! - <https://en.wiktionary.org/wiki/l%E1%BA%B9#Yoruba>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::NigerCongo, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Yoruba => "Yoruba" | "Èdè Yorùbá",
    dialects: [StandardYoruba => "Standard Yoruba"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Yoruba,
    categories: &[Natural],
    families: &[NigerCongo],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardYoruba,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ẹ n lẹ, Ayé!",
        }],
    }],
};
