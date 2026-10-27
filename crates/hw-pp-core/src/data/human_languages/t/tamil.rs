//! Hello-world greetings for `HumanLanguageName::Tamil`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%AE%B5%E0%AE%A3%E0%AE%95%E0%AF%8D%E0%AE%95%E0%AE%AE%E0%AF%8D#Tamil>
//! - <https://en.wiktionary.org/wiki/%E0%AE%89%E0%AE%B2%E0%AE%95%E0%AE%AE%E0%AF%8D#Tamil>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Dravidian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tamil => "Tamil" | "தமிழ்",
    dialects: [StandardTamil => "Standard Tamil"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tamil,
    categories: &[Natural],
    families: &[Dravidian],
    scripts: &[ScriptName::Tamil, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTamil,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Tamil],
                text: "வணக்கம், உலகம்!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "vaṇakkam, ulakam!",
            },
        ],
    }],
};
