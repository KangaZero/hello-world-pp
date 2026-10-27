//! Hello-world greetings for `HumanLanguageName::NorthernSami`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/bures#Northern_Sami>
//! - <https://en.wiktionary.org/wiki/m%C3%A1ilbmi#Northern_Sami>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Uralic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: NorthernSami => "Northern Sami" | "davvisámegiella",
    dialects: [StandardNorthernSami => "Standard Northern Sami"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::NorthernSami,
    categories: &[Natural],
    families: &[Uralic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardNorthernSami,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bures, máilbmi!",
        }],
    }],
};
