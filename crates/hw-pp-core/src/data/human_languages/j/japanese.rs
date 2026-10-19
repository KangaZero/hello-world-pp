//! Hello-world greetings for `HumanLanguageName::Japanese`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Japanese => "Japanese",
    dialects: [StandardJapanese => "Standard Japanese", KansaiJapanese => "Kansai Japanese"],
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
