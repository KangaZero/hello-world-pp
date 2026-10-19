//! Hello-world greetings for `HumanLanguageName::Kinyarwanda`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kinyarwanda => "Kinyarwanda" | "Ikinyarwanda",
    dialects: [StandardKinyarwanda => "Standard Kinyarwanda"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kinyarwanda,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKinyarwanda,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Muraho, isi!",
        }],
    }],
};
