//! Hello-world greetings for `HumanLanguageName::Telugu`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%B0%AA%E0%B1%8D%E0%B0%B0%E0%B0%AA%E0%B0%82%E0%B0%9A%E0%B0%AE%E0%B1%81#Telugu>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Dravidian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Telugu => "Telugu" | "తెలుగు",
    dialects: [StandardTelugu => "Standard Telugu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Telugu,
    categories: &[Natural],
    families: &[Dravidian],
    scripts: &[ScriptName::Telugu, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTelugu,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Telugu],
                text: "హలో, ప్రపంచం!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "halō, prapañcaṁ!",
            },
        ],
    }],
};
