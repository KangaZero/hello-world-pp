//! Hello-world greetings for `HumanLanguageName::Korean`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%EC%95%88%EB%85%95%ED%95%98%EC%84%B8%EC%9A%94#Korean>
//! - <https://en.wiktionary.org/wiki/%EC%84%B8%EA%B3%84#Korean>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Koreanic, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Korean => "Korean" | "한국어",
    dialects: [SouthKorean => "South Korean" | "표준어", NorthKorean => "North Korean" | "문화어"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Korean,
    categories: &[Natural],
    families: &[Koreanic],
    scripts: &[ScriptName::Hangul, ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::SouthKorean,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Hangul],
                    text: "안녕하세요, 세계!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "annyeonghaseyo, segye!",
                },
            ],
        },
        Dialect {
            label: DialectName::NorthKorean,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Hangul],
                    text: "안녕하세요, 세계!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "annyŏnghaseyo, segye!",
                },
            ],
        },
    ],
};
