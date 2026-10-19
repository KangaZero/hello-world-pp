//! Hello-world greetings for `HumanLanguageName::Afrikaans`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hallo#Afrikaans>
//! - <https://en.wiktionary.org/wiki/w%C3%AAreld#Afrikaans>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Afrikaans => "Afrikaans",
    dialects: [StandardAfrikaans => "Standard Afrikaans"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Afrikaans,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAfrikaans,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hallo, wêreld!",
        }],
    }],
};
