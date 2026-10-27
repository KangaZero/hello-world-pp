//! Hello-world greetings for `HumanLanguageName::Sinhala`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%B6%86%E0%B6%BA%E0%B7%94%E0%B6%B6%E0%B7%9D%E0%B7%80%E0%B6%B1%E0%B7%8A#Sinhalese>
//! - <https://en.wiktionary.org/wiki/%E0%B6%BD%E0%B7%9D%E0%B6%9A%E0%B6%BA#Sinhalese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sinhala => "Sinhala" | "සිංහල",
    dialects: [StandardSinhala => "Standard Sinhala"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sinhala,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Sinhala, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSinhala,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Sinhala],
                text: "ආයුබෝවන්, ලෝකය!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "āyubōvan, lōkaya!",
            },
        ],
    }],
};
