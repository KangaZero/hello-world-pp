//! Hello-world greetings for `HumanLanguageName::OldEnglish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/wes#Old_English>
//! - <https://en.wiktionary.org/wiki/weorold#Old_English>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Extinct,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: OldEnglish => "Old English" | "Englisc",
    dialects: [WestSaxon => "West Saxon"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::OldEnglish,
    categories: &[Extinct],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::WestSaxon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Wes hāl, weorold!",
        }],
    }],
};
