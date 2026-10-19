//! Hello-world greetings for `HumanLanguageName::Swedish`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Swedish => "Swedish" | "Svenska",
    dialects: [StandardSwedish => "Standard Swedish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Swedish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardSwedish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hej, världen!",
        }],
    }],
};
