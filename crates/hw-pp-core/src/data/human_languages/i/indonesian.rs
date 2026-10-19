//! Hello-world greetings for `HumanLanguageName::Indonesian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/halo#Indonesian>
//! - <https://en.wiktionary.org/wiki/dunia#Indonesian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Indonesian => "Indonesian" | "Bahasa Indonesia",
    dialects: [StandardIndonesian => "Standard Indonesian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Indonesian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIndonesian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halo, Dunia!",
        }],
    }],
};
