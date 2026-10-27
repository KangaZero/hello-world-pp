//! Hello-world greetings for `HumanLanguageName::Swedish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hej#Swedish>
//! - <https://en.wiktionary.org/wiki/v%C3%A4rlden#Swedish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Swedish => "Swedish" | "Svenska",
    dialects: [StandardSwedish => "Standard Swedish"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Swedish,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSwedish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hej, världen!",
        }],
    }],
};
