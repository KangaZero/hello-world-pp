//! Hello-world greetings for `HumanLanguageName::Danish`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Danish => "Danish",
    dialects: [StandardDanish => "Standard Danish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Danish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardDanish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hej, verden!",
        }],
    }],
};
