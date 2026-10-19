//! Hello-world greetings for `HumanLanguageName::Thai`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Thai => "Thai" | "ภาษาไทย",
    dialects: [CentralThai => "Central Thai" | "ภาษาไทยกลาง"],
    scripts: [Thai => "Thai"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Thai,
    categories: &[Natural],
    scripts: &[ScriptName::Thai],
    dialects: &[Dialect {
        label: DialectName::CentralThai,
        greetings: &[Greeting {
            scripts: &[ScriptName::Thai],
            text: "สวัสดีชาวโลก!",
        }],
    }],
};
