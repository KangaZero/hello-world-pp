//! Hello-world greetings for `HumanLanguageName::Hungarian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hungarian => "Hungarian" | "Magyar",
    dialects: [StandardHungarian => "Standard Hungarian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hungarian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardHungarian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Helló, világ!",
        }],
    }],
};
