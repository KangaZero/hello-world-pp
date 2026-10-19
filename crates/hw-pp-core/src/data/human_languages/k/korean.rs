//! Hello-world greetings for `HumanLanguageName::Korean`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Korean => "Korean" | "한국어",
    dialects: [SouthKorean => "South Korean" | "표준어", NorthKorean => "North Korean" | "문화어"],
    scripts: [Hangul => "Hangul"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Korean,
    categories: &[Natural],
    scripts: &[ScriptName::Hangul],
    dialects: &[
        Dialect {
            label: DialectName::SouthKorean,
            greetings: &[Greeting {
                scripts: &[ScriptName::Hangul],
                text: "안녕하세요, 세계!",
            }],
        },
        Dialect {
            label: DialectName::NorthKorean,
            greetings: &[Greeting {
                scripts: &[ScriptName::Hangul],
                text: "안녕하세요, 세계!",
            }],
        },
    ],
};
