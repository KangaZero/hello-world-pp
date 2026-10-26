//! Hello-world greetings for `HumanLanguageName::Indonesian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/halo#Indonesian>
//! - <https://en.wiktionary.org/wiki/dunia#Indonesian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austronesian, HumanLanguageName, ScriptName,
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
    families: &[Austronesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIndonesian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halo, Dunia!",
        }],
    }],
};
