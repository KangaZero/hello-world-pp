//! Hello-world greetings for `HumanLanguageName::Polish`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Polish => "Polish",
    dialects: [StandardPolish => "Standard Polish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Polish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardPolish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Witaj, świecie!",
        }],
    }],
};
