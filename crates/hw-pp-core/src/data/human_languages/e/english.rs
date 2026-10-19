//! Hello-world greetings for `HumanLanguageName::English`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hello#English>
//! - <https://en.wiktionary.org/wiki/g%27day#English>
//! - <https://en.wiktionary.org/wiki/world#English>
//! - <https://en.wiktionary.org/wiki/%E0%A4%B9%E0%A5%87%E0%A4%B2%E0%A5%8B#Hindi>
//! - <https://en.wiktionary.org/wiki/%E0%A6%B9%E0%A7%8D%E0%A6%AF%E0%A6%BE%E0%A6%B2%E0%A7%8B#Bengali>
//! - <https://en.wiktionary.org/wiki/%E0%AA%B9%E0%AB%87%E0%AA%B2%E0%AB%8B#Gujarati>
//! - <https://en.wiktionary.org/wiki/%E0%B4%B9%E0%B4%B2%E0%B5%8B#Malayalam>
//! - <https://en.wiktionary.org/wiki/%E0%B8%AE%E0%B8%B1%E0%B8%A5%E0%B9%82%E0%B8%AB%E0%B8%A5#Thai>
//! - <https://en.wiktionary.org/wiki/%E1%88%80%E1%88%8E#Amharic>
//! - <https://en.wiktionary.org/wiki/%D7%94%D7%9C%D7%95#Hebrew>
//! - <https://en.wiktionary.org/wiki/%D9%87%D9%84%D9%88#Iraqi_Arabic>
//! - <https://en.wiktionary.org/wiki/%D9%87%D9%84%D9%88#Persian>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: English => "English",
    dialects: [AmericanEnglish => "American English", BritishEnglish => "British English", AustralianEnglish => "Australian English"],
    scripts: [Latin => "Latin", Katakana => "Katakana", Kana => "Kana", Cyrillic => "Cyrillic", Hangul => "Hangul", Arabic => "Arabic", PersoArabic => "Perso-Arabic", Hebrew => "Hebrew", Devanagari => "Devanagari", Bengali => "Bengali", Gurmukhi => "Gurmukhi", Gujarati => "Gujarati", Odia => "Odia", Tamil => "Tamil", Telugu => "Telugu", Kannada => "Kannada", Malayalam => "Malayalam", Sinhala => "Sinhala", Thai => "Thai", Ethiopic => "Ethiopic", Braille => "Braille", BrailleContracted => "Braille (Contracted)"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::English,
    categories: &[Natural],
    scripts: &[
        ScriptName::Latin,
        ScriptName::Katakana,
        ScriptName::Kana,
        ScriptName::Cyrillic,
        ScriptName::Hangul,
        ScriptName::Arabic,
        ScriptName::PersoArabic,
        ScriptName::Hebrew,
        ScriptName::Devanagari,
        ScriptName::Bengali,
        ScriptName::Gurmukhi,
        ScriptName::Gujarati,
        ScriptName::Odia,
        ScriptName::Tamil,
        ScriptName::Telugu,
        ScriptName::Kannada,
        ScriptName::Malayalam,
        ScriptName::Sinhala,
        ScriptName::Thai,
        ScriptName::Ethiopic,
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
                    scripts: &[ScriptName::Katakana, ScriptName::Kana],
                    text: "ハローワールド！",
                },
                Greeting {
                    scripts: &[ScriptName::Cyrillic],
                    text: "Хелло, Ворлд!",
                },
                Greeting {
                    scripts: &[ScriptName::Hangul],
                    text: "헬로, 월드!",
                },
                // Iraqi Arabic هلو (from English); "world" transcribed (cf. ديزني وورلد)
                Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "هلو، وورلد!",
                },
                // Persian (Dari) هلو (from English); "world" transcribed (cf. دیزنی ورلد)
                Greeting {
                    scripts: &[ScriptName::PersoArabic],
                    text: "هلو، ورلد!",
                },
                // Hebrew הלו; "world" transcribed (cf. דיסני וורלד)
                Greeting {
                    scripts: &[ScriptName::Hebrew],
                    text: "הלו, וורלד!",
                },
                // Hindi हेलो (from English); "world" transcribed (cf. वर्ल्ड कप)
                Greeting {
                    scripts: &[ScriptName::Devanagari],
                    text: "हेलो, वर्ल्ड!",
                },
                // Bengali হ্যালো (from English); "world" transcribed (cf. ওয়ার্ল্ড কাপ)
                Greeting {
                    scripts: &[ScriptName::Bengali],
                    text: "হ্যালো, ওয়ার্ল্ড!",
                },
                // Punjabi transcription of English, no dictionary entry; cf. ਵਰਲਡ ਕੱਪ
                Greeting {
                    scripts: &[ScriptName::Gurmukhi],
                    text: "ਹੈਲੋ, ਵਰਲਡ!",
                },
                // Gujarati હેલો (from English); "world" transcribed (cf. વર્લ્ડ કપ)
                Greeting {
                    scripts: &[ScriptName::Gujarati],
                    text: "હેલો, વર્લ્ડ!",
                },
                // Odia transcription of English ("hello" as in Wiktionary's translation table); "world" transcribed
                Greeting {
                    scripts: &[ScriptName::Odia],
                    text: "ହ୍ୟାଲୋ, ୱାର୍ଲ୍ଡ!",
                },
                // Tamil transcription of English, no dictionary entry; cf. வேர்ல்ட் கப்
                Greeting {
                    scripts: &[ScriptName::Tamil],
                    text: "ஹலோ, வேர்ல்ட்!",
                },
                // Telugu transcription from the Hello World Collection
                Greeting {
                    scripts: &[ScriptName::Telugu],
                    text: "హలో, వరల్డ్!",
                },
                // Kannada transcription from the Hello World Collection
                Greeting {
                    scripts: &[ScriptName::Kannada],
                    text: "ಹಲೋ, ವರ್ಲ್ಡ್!",
                },
                // Malayalam ഹലോ (from English); "world" transcribed (cf. വേൾഡ് ബാങ്ക്)
                Greeting {
                    scripts: &[ScriptName::Malayalam],
                    text: "ഹലോ, വേൾഡ്!",
                },
                // Sinhala transcription of English ("hello" as in Wiktionary's translation table); "world" transcribed
                Greeting {
                    scripts: &[ScriptName::Sinhala],
                    text: "හලෝ, වර්ල්ඩ්!",
                },
                // Thai ฮัลโหล (from English); "world" transcribed (cf. เซ็นทรัลเวิลด์); Thai uses no comma
                Greeting {
                    scripts: &[ScriptName::Thai],
                    text: "ฮัลโหล เวิลด์!",
                },
                // Amharic ሀሎ (from English); "world" transcribed (cf. ወርልድ ባንክ); no comma, as `፣` is not stripped by the CLI
                Greeting {
                    scripts: &[ScriptName::Ethiopic],
                    text: "ሀሎ ወርልድ!",
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
