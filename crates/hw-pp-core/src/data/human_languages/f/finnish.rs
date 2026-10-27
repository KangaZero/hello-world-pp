//! Hello-world greetings for `HumanLanguageName::Finnish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hei#Finnish>
//! - <https://en.wiktionary.org/wiki/maailma#Finnish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Uralic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Finnish => "Finnish" | "Suomi",
    dialects: [StandardFinnish => "Standard Finnish"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Finnish,
    categories: &[Natural],
    families: &[Uralic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFinnish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hei, maailma!",
        }],
    }],
};
