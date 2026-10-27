//! Hello-world greetings for `HumanLanguageName::Sindhi`.
//!
//! Sources:
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
    language: Sindhi => "Sindhi" | "سنڌي",
    dialects: [StandardSindhi => "Standard Sindhi"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sindhi,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::PersoArabic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSindhi,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::PersoArabic],
                text: "هيلو، دنيا!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "helo, dunyā!",
            },
        ],
    }],
};
