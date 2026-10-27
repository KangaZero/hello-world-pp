//! Hello-world greetings for `HumanLanguageName::AncientGreek`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%CF%87%CE%B1%E1%BF%96%CF%81%CE%B5#Ancient_Greek>
//! - <https://en.wiktionary.org/wiki/%CE%BA%CF%8C%CF%83%CE%BC%CE%B5#Ancient_Greek>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Dead,
    HumanLanguageFamily::{Hellenic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: AncientGreek => "Ancient Greek" | "Ἑλληνική",
    dialects: [AtticGreek => "Attic Greek"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::AncientGreek,
    categories: &[Dead],
    families: &[IndoEuropean, Hellenic],
    scripts: &[ScriptName::Greek, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::AtticGreek,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Greek],
                text: "Χαῖρε, κόσμε!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Khaîre, kósme!",
            },
        ],
    }],
};
