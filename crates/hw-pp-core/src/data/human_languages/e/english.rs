//! Hello-world greetings for `HumanLanguageName::English`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: English => "English",
    dialects: [AmericanEnglish => "American English", BritishEnglish => "British English", AustralianEnglish => "Australian English"],
    scripts: [Latin => "Latin", Katakana => "Katakana", Cyrillic => "Cyrillic", Hangul => "Hangul", Braille => "Braille", BrailleContracted => "Braille (Contracted)"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::English,
    categories: &[Natural],
    scripts: &[
        ScriptName::Latin,
        ScriptName::Katakana,
        ScriptName::Cyrillic,
        ScriptName::Hangul,
        ScriptName::Braille,
        ScriptName::BrailleContracted,
    ],
    dialects: &[
        Dialect {
            label: DialectName::AmericanEnglish,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Hello, World!",
                },
                Greeting {
                    scripts: &[ScriptName::Katakana],
                    text: "ハローワールド",
                },
                Greeting {
                    scripts: &[ScriptName::Cyrillic],
                    text: "Хелло, Ворлд",
                },
                Greeting {
                    scripts: &[ScriptName::Hangul],
                    text: "헬로, 월드!",
                },
                // Unified English Braille, uncontracted (Grade 1); ⠠ = capital indicator
                Greeting {
                    scripts: &[ScriptName::Braille],
                    text: "⠠⠓⠑⠇⠇⠕⠂ ⠠⠺⠕⠗⠇⠙⠖",
                },
                // Unified English Braille, contracted (Grade 2); ⠸⠺ = "world"
                Greeting {
                    scripts: &[ScriptName::BrailleContracted],
                    text: "⠠⠓⠑⠇⠇⠕⠂ ⠠⠸⠺⠖",
                },
            ],
        },
        Dialect {
            label: DialectName::BritishEnglish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hello, World!",
            }],
        },
        Dialect {
            label: DialectName::AustralianEnglish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "G'day, World!",
            }],
        },
    ],
};
