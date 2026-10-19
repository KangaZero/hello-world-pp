//! Hello-world greetings for `HumanLanguageName::Latin`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Dead, HumanLanguageName,
    ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Latin => "Latin",
    dialects: [ClassicalLatin => "Classical Latin"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Latin,
    categories: &[Dead],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ClassicalLatin,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salve, munde!",
        }],
    }],
};
