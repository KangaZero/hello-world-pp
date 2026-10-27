//! Hello-world greetings for `HumanLanguageName::Turkmen`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/d%C3%BCn%C3%BD%C3%A4#Turkmen>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Turkic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Turkmen => "Turkmen" | "türkmençe",
    dialects: [StandardTurkmen => "Standard Turkmen"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Turkmen,
    categories: &[Natural],
    families: &[Turkic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTurkmen,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salam, dünýä!",
        }],
    }],
};
