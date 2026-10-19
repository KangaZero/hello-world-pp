//! Hello-world greetings for `HumanLanguageName::Quechua`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/pacha#Quechua>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Quechua => "Quechua" | "Runa Simi",
    dialects: [SouthernQuechua => "Southern Quechua"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Quechua,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::SouthernQuechua,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Napaykullayki, pacha!",
        }],
    }],
};
