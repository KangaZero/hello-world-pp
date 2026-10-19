//! Hello-world greetings for `HumanLanguageName::Hebrew`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D7%A9%D7%9C%D7%95%D7%9D#Hebrew>
//! - <https://en.wiktionary.org/wiki/%D7%A2%D7%95%D7%9C%D7%9D#Hebrew>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hebrew => "Hebrew" | "עברית",
    dialects: [ModernHebrew => "Modern Hebrew"],
    scripts: [Hebrew => "Hebrew"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hebrew,
    categories: &[Natural],
    scripts: &[ScriptName::Hebrew],
    dialects: &[Dialect {
        label: DialectName::ModernHebrew,
        greetings: &[Greeting {
            scripts: &[ScriptName::Hebrew],
            text: "שלום, עולם!",
        }],
    }],
};
