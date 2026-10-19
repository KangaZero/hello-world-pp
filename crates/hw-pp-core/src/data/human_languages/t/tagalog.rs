//! Hello-world greetings for `HumanLanguageName::Tagalog`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kumusta#Tagalog>
//! - <https://en.wiktionary.org/wiki/mundo#Tagalog>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tagalog => "Tagalog" | "Filipino",
    dialects: [StandardTagalog => "Standard Tagalog"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tagalog,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTagalog,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kumusta, mundo!",
        }],
    }],
};
