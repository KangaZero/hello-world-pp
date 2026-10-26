//! Hello-world greetings for `HumanLanguageName::Greek`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%CE%B3%CE%B5%CE%B9%CE%B1#Greek>
//! - <https://en.wiktionary.org/wiki/%CF%83%CE%BF%CF%85#Greek>
//! - <https://en.wiktionary.org/wiki/%CE%BA%CF%8C%CF%83%CE%BC%CE%B5#Greek>
//! - <https://en.wiktionary.org/wiki/%CE%B3%CE%B5%CE%B9%CE%B1_%CF%83%CE%BF%CF%85#Greek>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Hellenic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Greek => "Greek" | "Ελληνικά",
    dialects: [ModernGreek => "Modern Greek"],
    scripts: [Greek => "Greek", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Greek,
    categories: &[Natural],
    families: &[IndoEuropean, Hellenic],
    scripts: &[ScriptName::Greek, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ModernGreek,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Greek],
                text: "Γεια σου, Κόσμε!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Geia sou, Kósme!",
            },
        ],
    }],
};
