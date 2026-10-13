use crate::data::common::{DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT};
use crate::data::lookup::LookupError;
use crate::data::macros::name_enum;

use HumanLanguageCategory::{Constructed, Dead, Extinct, Natural, Signed};
use thiserror::Error;

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum HumanLanguageCategory: ParseHumanLanguageCategoryError("category") {
        Natural => "Natural",
        Constructed => "Constructed",
        Signed => "Signed",
        /// See <https://www.globallanguageservices.co.uk/difference-extinct-language-dead-language/>
        Dead => "Dead",
        Extinct => "Extinct",
    }
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash )]
    pub enum ScriptName: ParseScriptNameError("script") {
        Latin => "Latin",
        Arabic => "Arabic",
        Armenian => "Armenian",
        Bengali => "Bengali",
        Braille => "Braille",
        BrailleContracted => "Braille (Contracted)",
        Cuneiform => "Cuneiform",
        Cyrillic => "Cyrillic",
        Devanagari => "Devanagari",
        Georgian => "Georgian",
        Greek => "Greek",
        Han => "Han",
        Pinyin => "Pinyin",
        SimplifiedChinese => "Simplified Chinese",
        TraditionalChinese => "Traditional Chinese",
        Hangul => "Hangul",
        Hebrew => "Hebrew",
        /// Hepburn style
        Romaji => "Romaji",
        Kanji => "Kanji",
        Hiragana => "Hiragana",
        Katakana => "Katakana",
        Kana => "Kana",
        Piqad => "pIqaD",
        PersoArabic => "Perso-Arabic",
        Tamil => "Tamil",
        Telugu => "Telugu",
        Thai => "Thai",
    }
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum HumanLanguageName: ParseHumanLanguageNameError("language") {
        #[default]
        English => "English",
        Chinese => "Chinese",
        Spanish => "Spanish",
        Hindi => "Hindi",
        Arabic => "Arabic",
        Bengali => "Bengali",
        Portuguese => "Portuguese",
        Russian => "Russian",
        Japanese => "Japanese",
        German => "German",
        French => "French",
        Italian => "Italian",
        Korean => "Korean",
        Turkish => "Turkish",
        Vietnamese => "Vietnamese",
        Polish => "Polish",
        Ukrainian => "Ukrainian",
        Dutch => "Dutch",
        Thai => "Thai",
        Greek => "Greek",
        Hebrew => "Hebrew",
        Swedish => "Swedish",
        Indonesian => "Indonesian",
        Czech => "Czech",
        Finnish => "Finnish",
        Danish => "Danish",
        Norwegian => "Norwegian",
        Hungarian => "Hungarian",
        Persian => "Persian",
        Swahili => "Swahili",
        Romanian => "Romanian",
        Tamil => "Tamil",
        Telugu => "Telugu",
        Armenian => "Armenian",
        Georgian => "Georgian",
        Yoruba => "Yoruba",
        Zulu => "Zulu",
        Esperanto => "Esperanto",
        Lojban => "Lojban",
        Klingon => "Klingon",
        Latin => "Latin",
        Sanskrit => "Sanskrit",
        AncientGreek => "Ancient Greek",
        Sumerian => "Sumerian",
        AmericanSignLanguage => "American Sign Language",
        BritishSignLanguage => "British Sign Language",
    }
}

name_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum DialectName: ParseDialectNameError("dialect") {
        #[default]
        AmericanEnglish => "American English",
        BritishEnglish => "British English",
        AustralianEnglish => "Australian English",
        Mandarin => "Mandarin",
        Cantonese => "Cantonese",
        Hokkien => "Hokkien",
        CastilianSpanish => "Castilian Spanish",
        MexicanSpanish => "Mexican Spanish",
        RioplatenseSpanish => "Rioplatense Spanish",
        ModernStandardHindi => "Modern Standard Hindi",
        ModernStandardArabic => "Modern Standard Arabic",
        EgyptianArabic => "Egyptian Arabic",
        LevantineArabic => "Levantine Arabic",
        StandardBengali => "Standard Bengali",
        BrazilianPortuguese => "Brazilian Portuguese",
        EuropeanPortuguese => "European Portuguese",
        StandardRussian => "Standard Russian",
        StandardJapanese => "Standard Japanese",
        KansaiJapanese => "Kansai Japanese",
        StandardGerman => "Standard German",
        SwissGerman => "Swiss German",
        Bavarian => "Bavarian",
        MetropolitanFrench => "Metropolitan French",
        QuebecoisFrench => "Québécois French",
        StandardItalian => "Standard Italian",
        SouthKorean => "South Korean",
        NorthKorean => "North Korean",
        IstanbulTurkish => "Istanbul Turkish",
        StandardVietnamese => "Standard Vietnamese",
        StandardPolish => "Standard Polish",
        StandardUkrainian => "Standard Ukrainian",
        StandardDutch => "Standard Dutch",
        CentralThai => "Central Thai",
        ModernGreek => "Modern Greek",
        ModernHebrew => "Modern Hebrew",
        StandardSwedish => "Standard Swedish",
        StandardIndonesian => "Standard Indonesian",
        StandardCzech => "Standard Czech",
        StandardFinnish => "Standard Finnish",
        StandardDanish => "Standard Danish",
        Bokmal => "Bokmål",
        Nynorsk => "Nynorsk",
        StandardHungarian => "Standard Hungarian",
        IranianPersian => "Iranian Persian",
        StandardSwahili => "Standard Swahili",
        StandardRomanian => "Standard Romanian",
        StandardTamil => "Standard Tamil",
        StandardTelugu => "Standard Telugu",
        EasternArmenian => "Eastern Armenian",
        WesternArmenian => "Western Armenian",
        StandardGeorgian => "Standard Georgian",
        StandardYoruba => "Standard Yoruba",
        StandardZulu => "Standard Zulu",
        StandardEsperanto => "Standard Esperanto",
        StandardLojban => "Standard Lojban",
        StandardKlingon => "Standard Klingon",
        ClassicalLatin => "Classical Latin",
        ClassicalSanskrit => "Classical Sanskrit",
        AtticGreek => "Attic Greek",
        Emegir => "Emegir",
        Asl => "ASL",
        Bsl => "BSL",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HumanLanguage {
    pub name: HumanLanguageName,
    pub categories: &'static [HumanLanguageCategory],
    pub scripts: &'static [ScriptName], // scripts used by the language overall
    pub dialects: &'static [Dialect],
}

/// A spoken variety of a language (how it sounds), e.g. "Mandarin" vs "Cantonese".
#[derive(Debug, Clone, Copy)]
pub struct Dialect {
    pub label: DialectName,
    /// The same greeting written in different scripts. The first entry is the default.
    pub greetings: &'static [Greeting],
}

/// One written form of a dialect's greeting.
#[derive(Debug, Clone, Copy)]
pub struct Greeting {
    /// Scripts used by `text`; may hold several (Japanese mixes Kanji and Hiragana),
    /// or none for languages with no written form.
    pub scripts: &'static [ScriptName],
    pub text: &'static str,
}

pub static HUMAN_LANGUAGES: &[HumanLanguage] = &[
    HumanLanguage {
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
    },
    HumanLanguage {
        name: HumanLanguageName::Chinese,
        categories: &[Natural],
        scripts: &[
            ScriptName::Han,
            ScriptName::SimplifiedChinese,
            ScriptName::TraditionalChinese,
            ScriptName::Latin,
            ScriptName::Pinyin,
        ],
        dialects: &[
            Dialect {
                label: DialectName::Mandarin,
                greetings: &[
                    Greeting {
                        scripts: &[ScriptName::SimplifiedChinese],
                        text: "你好，世界！",
                    },
                    Greeting {
                        scripts: &[ScriptName::Latin, ScriptName::Pinyin],
                        text: "Nǐ hǎo, shìjiè!",
                    },
                ],
            },
            Dialect {
                label: DialectName::Cantonese,
                greetings: &[Greeting {
                    scripts: &[ScriptName::TraditionalChinese],
                    text: "哈囉，世界！",
                }],
            },
            Dialect {
                label: DialectName::Hokkien,
                greetings: &[Greeting {
                    scripts: &[ScriptName::TraditionalChinese],
                    text: "汝好，世界！",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Spanish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::CastilianSpanish,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "¡Hola, Mundo!",
                }],
            },
            Dialect {
                label: DialectName::MexicanSpanish,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "¡Hola, Mundo!",
                }],
            },
            Dialect {
                label: DialectName::RioplatenseSpanish,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "¡Hola, Mundo!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Hindi,
        categories: &[Natural],
        scripts: &[ScriptName::Devanagari],
        dialects: &[Dialect {
            label: DialectName::ModernStandardHindi,
            greetings: &[Greeting {
                scripts: &[ScriptName::Devanagari],
                text: "नमस्ते दुनिया!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Arabic,
        categories: &[Natural],
        scripts: &[ScriptName::Arabic],
        dialects: &[
            Dialect {
                label: DialectName::ModernStandardArabic,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "مرحبا بالعالم!",
                }],
            },
            Dialect {
                label: DialectName::EgyptianArabic,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "أهلاً بالعالم!",
                }],
            },
            Dialect {
                label: DialectName::LevantineArabic,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "مرحبا بالعالم!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Bengali,
        categories: &[Natural],
        scripts: &[ScriptName::Bengali],
        dialects: &[Dialect {
            label: DialectName::StandardBengali,
            greetings: &[Greeting {
                scripts: &[ScriptName::Bengali],
                text: "হ্যালো, বিশ্ব!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Portuguese,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::BrazilianPortuguese,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Olá, Mundo!",
                }],
            },
            Dialect {
                label: DialectName::EuropeanPortuguese,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Olá, Mundo!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Russian,
        categories: &[Natural],
        scripts: &[ScriptName::Cyrillic],
        dialects: &[Dialect {
            label: DialectName::StandardRussian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Привет, мир!",
            }],
        }],
    },
    HumanLanguage {
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
    },
    HumanLanguage {
        name: HumanLanguageName::German,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::StandardGerman,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Hallo, Welt!",
                }],
            },
            Dialect {
                label: DialectName::SwissGerman,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Hallo, Wält!",
                }],
            },
            Dialect {
                label: DialectName::Bavarian,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Servus, Wöd!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::French,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::MetropolitanFrench,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Bonjour le monde!",
                }],
            },
            Dialect {
                label: DialectName::QuebecoisFrench,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Bonjour le monde!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Italian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardItalian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Ciao, mondo!",
            }],
        }],
    },
    HumanLanguage {
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
    },
    HumanLanguage {
        name: HumanLanguageName::Turkish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::IstanbulTurkish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Merhaba, Dünya!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Vietnamese,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardVietnamese,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Xin chào, thế giới!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Polish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardPolish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Witaj, świecie!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Ukrainian,
        categories: &[Natural],
        scripts: &[ScriptName::Cyrillic],
        dialects: &[Dialect {
            label: DialectName::StandardUkrainian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Привіт, світе!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Dutch,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardDutch,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hallo, wereld!",
            }],
        }],
    },
    HumanLanguage {
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
    },
    HumanLanguage {
        name: HumanLanguageName::Greek,
        categories: &[Natural],
        scripts: &[ScriptName::Greek],
        dialects: &[Dialect {
            label: DialectName::ModernGreek,
            greetings: &[Greeting {
                scripts: &[ScriptName::Greek],
                text: "Γεια σου, Κόσμε!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Hebrew,
        categories: &[Natural],
        scripts: &[ScriptName::Hebrew],
        dialects: &[Dialect {
            label: DialectName::ModernHebrew,
            greetings: &[Greeting {
                scripts: &[ScriptName::Hebrew],
                text: "שלום, עולם!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Swedish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardSwedish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hej, världen!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Indonesian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardIndonesian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Halo, Dunia!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Czech,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardCzech,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Ahoj, světe!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Finnish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardFinnish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hei, maailma!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Danish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardDanish,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hej, verden!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Norwegian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::Bokmal,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Hei, verden!",
                }],
            },
            Dialect {
                label: DialectName::Nynorsk,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Hei, verda!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Hungarian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardHungarian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Helló, világ!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Persian,
        categories: &[Natural],
        scripts: &[ScriptName::PersoArabic],
        dialects: &[Dialect {
            label: DialectName::IranianPersian,
            greetings: &[Greeting {
                scripts: &[ScriptName::PersoArabic],
                text: "سلام دنیا!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Swahili,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardSwahili,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Habari, Dunia!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Romanian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardRomanian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salut, lume!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Tamil,
        categories: &[Natural],
        scripts: &[ScriptName::Tamil],
        dialects: &[Dialect {
            label: DialectName::StandardTamil,
            greetings: &[Greeting {
                scripts: &[ScriptName::Tamil],
                text: "வணக்கம், உலகம்!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Telugu,
        categories: &[Natural],
        scripts: &[ScriptName::Telugu],
        dialects: &[Dialect {
            label: DialectName::StandardTelugu,
            greetings: &[Greeting {
                scripts: &[ScriptName::Telugu],
                text: "హలో, ప్రపంచం!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Armenian,
        categories: &[Natural],
        scripts: &[ScriptName::Armenian],
        dialects: &[
            Dialect {
                label: DialectName::EasternArmenian,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Armenian],
                    text: "Բարեւ, աշխարհ!",
                }],
            },
            Dialect {
                label: DialectName::WesternArmenian,
                greetings: &[Greeting {
                    scripts: &[ScriptName::Armenian],
                    text: "Բարեւ աշխարհ!",
                }],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Georgian,
        categories: &[Natural],
        scripts: &[ScriptName::Georgian],
        dialects: &[Dialect {
            label: DialectName::StandardGeorgian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Georgian],
                text: "გამარჯობა, მსოფლიო!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Yoruba,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardYoruba,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Ẹ n lẹ, Ayé!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Zulu,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardZulu,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Sawubona, Mhlaba!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Esperanto,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardEsperanto,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Saluton, mondo!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Lojban,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardLojban,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "coi munje",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Klingon,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin, ScriptName::Piqad],
        dialects: &[Dialect {
            label: DialectName::StandardKlingon,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "nuqneH, qo'!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Latin,
        categories: &[Dead],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::ClassicalLatin,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salve, munde!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Sanskrit,
        categories: &[Natural],
        scripts: &[ScriptName::Devanagari],
        dialects: &[Dialect {
            label: DialectName::ClassicalSanskrit,
            greetings: &[Greeting {
                scripts: &[ScriptName::Devanagari],
                text: "नमस्ते विश्व!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::AncientGreek,
        categories: &[Dead],
        scripts: &[ScriptName::Greek],
        dialects: &[Dialect {
            label: DialectName::AtticGreek,
            greetings: &[Greeting {
                scripts: &[ScriptName::Greek],
                text: "Χαῖρε, κόσμε!",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Sumerian,
        categories: &[Extinct],
        scripts: &[ScriptName::Cuneiform],
        dialects: &[Dialect {
            label: DialectName::Emegir,
            greetings: &[
                // silim an-ki: "well-being, heaven-earth"; a reconstruction, not attested
                Greeting {
                    scripts: &[ScriptName::Cuneiform],
                    text: "𒁲 𒀭𒆠",
                },
            ],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::AmericanSignLanguage,
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: DialectName::Asl,
            greetings: &[Greeting {
                scripts: &[],
                text: "HELLO WORLD",
            }],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::BritishSignLanguage,
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: DialectName::Bsl,
            greetings: &[Greeting {
                scripts: &[],
                text: "HELLO WORLD",
            }],
        }],
    },
];

/// Language/dialect resolution errors for the human-language catalog.
pub type HumanLookupError = LookupError<HumanLanguageName, DialectName>;

/// Errors that can occur while looking up localized greetings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum HumanLanguageError {
    #[error(transparent)]
    Lookup(#[from] HumanLookupError),

    #[error("no 'hello world' found for {language} ({dialect}) written in {script}")]
    UnknownScript {
        language: HumanLanguageName,
        dialect: DialectName,
        script: ScriptName,
    },

    #[error("no 'hello world' recorded for {language} ({dialect})")]
    MissingGreeting {
        language: HumanLanguageName,
        dialect: DialectName,
    },
}

/// Look up the localized greeting for a language and dialect.
///
/// If both `language` and `dialect` are `None`, returns the default greeting.
/// If only `language` is given, its first dialect is used.
/// If only `dialect` is given, the language is inferred from whichever
/// language records that dialect.
/// If `script` is `None`, the dialect's first greeting is used; otherwise the
/// first greeting written in that script.
///
/// # Errors
///
/// Returns a [`HumanLanguageError`] if the language is not recorded, if the
/// dialect is not one of the language's dialects, if a dialect given on its
/// own belongs to no recorded language, or if no greeting is written in `script`.
pub fn get_human_language_hello_world_for(
    language: Option<HumanLanguageName>,
    dialect: Option<DialectName>,
    script: Option<ScriptName>,
) -> Result<&'static str, HumanLanguageError> {
    let (resolved_language, resolved_dialect) = match (language, dialect) {
        (None, None) => (DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT),
        (None, Some(existing_dialect)) => (
            get_human_language_from_dialect(existing_dialect)?,
            existing_dialect,
        ),
        (Some(existing_lang), None) => (
            existing_lang,
            get_default_human_language_dialect_from_language(existing_lang)?,
        ),
        (Some(existing_lang), Some(existing_dialect)) => (existing_lang, existing_dialect),
    };

    let lang = HUMAN_LANGUAGES
        .iter()
        .find(|lang| lang.name == resolved_language)
        .ok_or(LookupError::UnknownLanguage(resolved_language))?;
    let lang_dialect = lang
        .dialects
        .iter()
        .find(|d| d.label == resolved_dialect)
        .ok_or(LookupError::UnknownVariant {
            language: resolved_language,
            variant: resolved_dialect,
        })?;

    let greeting = match script {
        None => lang_dialect
            .greetings
            .first()
            .ok_or(HumanLanguageError::MissingGreeting {
                language: resolved_language,
                dialect: resolved_dialect,
            })?,
        Some(wanted) => lang_dialect
            .greetings
            .iter()
            .find(|greeting| greeting.scripts.contains(&wanted))
            .ok_or(HumanLanguageError::UnknownScript {
                language: resolved_language,
                dialect: resolved_dialect,
                script: wanted,
            })?,
    };
    Ok(greeting.text)
}

/// Finds the language that records `dialect`.
///
/// # Errors
///
/// Returns [`LookupError::NoOwner`] if no recorded language has
/// the dialect.
pub fn get_human_language_from_dialect(
    dialect: DialectName,
) -> Result<HumanLanguageName, HumanLookupError> {
    HUMAN_LANGUAGES
        .iter()
        .find(|lang| lang.dialects.iter().any(|d| d.label == dialect))
        .map(|lang| lang.name)
        .ok_or(LookupError::NoOwner(dialect))
}

/// Defaults to the 1st in the array of the set `language` parameter
///
/// # Errors
///
/// Returns [`LookupError::UnknownLanguage`] if the language is not recorded,
/// or [`LookupError::NoVariants`] if it has no dialects.
pub fn get_default_human_language_dialect_from_language(
    language: HumanLanguageName,
) -> Result<DialectName, HumanLookupError> {
    let lang = HUMAN_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(LookupError::UnknownLanguage(language))?;
    lang.dialects
        .first()
        .map(|dialect| dialect.label)
        .ok_or(LookupError::NoVariants(language))
}

#[must_use]
pub fn get_all_human_language_names() -> Vec<HumanLanguageName> {
    HUMAN_LANGUAGES.iter().map(|lang| lang.name).collect()
}

#[must_use]
pub fn get_all_human_languages_by_category(
    category: HumanLanguageCategory,
) -> Vec<&'static HumanLanguage> {
    HUMAN_LANGUAGES
        .iter()
        .filter(|lang| lang.categories.contains(&category))
        .collect()
}

#[must_use]
pub fn get_all_human_languages_by_script(script: ScriptName) -> Vec<&'static HumanLanguage> {
    HUMAN_LANGUAGES
        .iter()
        .filter(|lang| lang.scripts.contains(&script))
        .collect()
}
