//! Hello-world greetings for `HumanLanguageName::AncientGreek`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Dead, HumanLanguageName,
    ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: AncientGreek => "Ancient Greek" | "Ἑλληνική",
    dialects: [AtticGreek => "Attic Greek"],
    scripts: [Greek => "Greek"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::AncientGreek,
    categories: &[Dead],
    scripts: &[ScriptName::Greek],
    dialects: &[Dialect {
        label: DialectName::AtticGreek,
        greetings: &[Greeting {
            scripts: &[ScriptName::Greek],
            text: "Χαῖρε, κόσμε!",
        }],
    }],
};
