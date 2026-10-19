//! Hello-world greetings for `HumanLanguageName::Tamil`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tamil => "Tamil",
    dialects: [StandardTamil => "Standard Tamil"],
    scripts: [Tamil => "Tamil"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tamil,
    categories: &[Natural],
    scripts: &[ScriptName::Tamil],
    dialects: &[Dialect {
        label: DialectName::StandardTamil,
        greetings: &[Greeting {
            scripts: &[ScriptName::Tamil],
            text: "வணக்கம், உலகம்!",
        }],
    }],
};
