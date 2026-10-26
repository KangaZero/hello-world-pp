//! Hello-world greetings for `HumanLanguageName::Thai`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%B8%AA%E0%B8%A7%E0%B8%B1%E0%B8%AA%E0%B8%94%E0%B8%B5#Thai>
//! - <https://en.wiktionary.org/wiki/%E0%B8%8A%E0%B8%B2%E0%B8%A7#Thai>
//! - <https://en.wiktionary.org/wiki/%E0%B9%82%E0%B8%A5%E0%B8%81#Thai>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Thai => "Thai" | "ภาษาไทย",
    dialects: [CentralThai => "Central Thai" | "ภาษาไทยกลาง"],
    scripts: [Thai => "Thai", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Thai,
    categories: &[Natural],
    scripts: &[ScriptName::Thai, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::CentralThai,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Thai],
                text: "สวัสดีชาวโลก!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "sawatdi chao lok!",
            },
        ],
    }],
};
