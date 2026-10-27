//! Hello-world greetings for `HumanLanguageName::Amharic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%88%B0%E1%88%8B%E1%88%9D#Amharic>
//! - <https://en.wiktionary.org/wiki/%E1%8B%93%E1%88%88%E1%88%9D#Amharic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Amharic => "Amharic" | "አማርኛ",
    dialects: [StandardAmharic => "Standard Amharic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Amharic,
    categories: &[Natural],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Ethiopic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAmharic,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Ethiopic],
                text: "ሰላም ዓለም!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Selam, alem!",
            },
        ],
    }],
};
