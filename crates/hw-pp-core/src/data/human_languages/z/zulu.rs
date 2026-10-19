//! Hello-world greetings for `HumanLanguageName::Zulu`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Zulu => "Zulu",
    dialects: [StandardZulu => "Standard Zulu"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Zulu,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardZulu,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Sawubona, Mhlaba!",
        }],
    }],
};
