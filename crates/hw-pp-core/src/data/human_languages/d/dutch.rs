//! Hello-world greetings for `HumanLanguageName::Dutch`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Dutch => "Dutch",
    dialects: [StandardDutch => "Standard Dutch"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Dutch,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardDutch,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hallo, wereld!",
        }],
    }],
};
