//! Hello-world greetings for `HumanLanguageName::Yoruba`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Yoruba => "Yoruba",
    dialects: [StandardYoruba => "Standard Yoruba"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Yoruba,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardYoruba,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ẹ n lẹ, Ayé!",
        }],
    }],
};
