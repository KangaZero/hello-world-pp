//! Hello-world greetings for `HumanLanguageName::Japanese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/konnichiwa#Japanese>
//! - <https://en.wiktionary.org/wiki/%E3%81%BE%E3%81%84%E3%81%A9#Japanese>
//! - <https://en.wiktionary.org/wiki/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF#Japanese>
//! - <https://en.wiktionary.org/wiki/%E4%B8%96%E7%95%8C#Japanese>
//! - <https://en.wiktionary.org/wiki/sekai#Japanese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Japanese => "Japanese" | "日本語",
    dialects: [StandardJapanese => "Standard Japanese" | "標準語", KansaiJapanese => "Kansai Japanese" | "関西弁"],
    scripts: [Kanji => "Kanji", Hiragana => "Hiragana", Katakana => "Katakana", Kana => "Kana", Latin => "Latin", Romaji => "Romaji"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Japanese,
    categories: &[Natural],
    scripts: &[
        ScriptName::Kanji,
        ScriptName::Hiragana,
        ScriptName::Katakana,
        ScriptName::Kana,
        ScriptName::Latin,
        ScriptName::Romaji,
    ],
    dialects: &[
        Dialect {
            label: DialectName::StandardJapanese,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Kana, ScriptName::Kanji, ScriptName::Hiragana],
                    text: "こんにちは世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Romaji, ScriptName::Latin],
                    text: "konnichiwa sekai!",
                },
            ],
        },
        Dialect {
            label: DialectName::KansaiJapanese,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Kana, ScriptName::Kanji, ScriptName::Hiragana],
                    text: "まいど、世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Romaji, ScriptName::Latin],
                    text: "maido, sekai!",
                },
            ],
        },
    ],
};
