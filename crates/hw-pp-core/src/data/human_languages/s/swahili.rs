//! Hello-world greetings for `HumanLanguageName::Swahili`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Swahili => "Swahili" | "Kiswahili",
    dialects: [StandardSwahili => "Standard Swahili" | "Kiswahili sanifu"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Swahili,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSwahili,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Habari, Dunia!",
        }],
    }],
};
