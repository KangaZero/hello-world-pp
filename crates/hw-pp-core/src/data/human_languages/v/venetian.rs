//! Hello-world greetings for `HumanLanguageName::Venetian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mondo#Venetan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Venetian => "Venetian" | "vèneto",
    dialects: [StandardVenetian => "Standard Venetian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Venetian,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardVenetian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ciao, mondo!",
        }],
    }],
};
