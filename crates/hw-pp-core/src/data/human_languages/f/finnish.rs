//! Hello-world greetings for `HumanLanguageName::Finnish`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Finnish => "Finnish",
    dialects: [StandardFinnish => "Standard Finnish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Finnish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFinnish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hei, maailma!",
        }],
    }],
};
