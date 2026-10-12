use super::common::HumanLanguageCategory;
use HumanLanguageCategory::{Constructed, Extinct, Natural, Signed};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScriptName {
    #[default]
    Latin,
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Georgian,
    Greek,
    Han,
    SimplifiedChinese,
    TraditionalChinese,
    Hangul,
    Hebrew,
    Kanji,
    Hiragana,
    Katakana,
    Piqad,
    PersoArabic,
    Tamil,
    Telugu,
    Thai,
}

impl ScriptName {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Arabic => "Arabic",
            Self::Armenian => "Armenian",
            Self::Bengali => "Bengali",
            Self::Cyrillic => "Cyrillic",
            Self::Devanagari => "Devanagari",
            Self::Georgian => "Georgian",
            Self::Greek => "Greek",
            Self::Han => "Han",
            Self::SimplifiedChinese => "Simplified Chinese",
            Self::TraditionalChinese => "Traditional Chinese",
            Self::Hangul => "Hangul",
            Self::Hebrew => "Hebrew",
            Self::Kanji => "Kanji",
            Self::Hiragana => "Hiragana",
            Self::Katakana => "Katakana",
            Self::Latin => "Latin",
            Self::Piqad => "pIqaD",
            Self::PersoArabic => "Perso-Arabic",
            Self::Tamil => "Tamil",
            Self::Telugu => "Telugu",
            Self::Thai => "Thai",
        }
    }
}

impl core::fmt::Display for ScriptName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HumanLanguageName {
    #[default]
    English,
    Chinese,
    Spanish,
    Hindi,
    Arabic,
    Bengali,
    Portuguese,
    Russian,
    Japanese,
    German,
    French,
    Italian,
    Korean,
    Turkish,
    Vietnamese,
    Polish,
    Ukrainian,
    Dutch,
    Thai,
    Greek,
    Hebrew,
    Swedish,
    Indonesian,
    Czech,
    Finnish,
    Danish,
    Norwegian,
    Hungarian,
    Persian,
    Swahili,
    Romanian,
    Tamil,
    Telugu,
    Armenian,
    Georgian,
    Yoruba,
    Zulu,
    Esperanto,
    Lojban,
    Klingon,
    Latin,
    Sanskrit,
    AncientGreek,
    AmericanSignLanguage,
    BritishSignLanguage,
}

impl HumanLanguageName {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "Chinese",
            Self::Spanish => "Spanish",
            Self::Hindi => "Hindi",
            Self::Arabic => "Arabic",
            Self::Bengali => "Bengali",
            Self::Portuguese => "Portuguese",
            Self::Russian => "Russian",
            Self::Japanese => "Japanese",
            Self::German => "German",
            Self::French => "French",
            Self::Italian => "Italian",
            Self::Korean => "Korean",
            Self::Turkish => "Turkish",
            Self::Vietnamese => "Vietnamese",
            Self::Polish => "Polish",
            Self::Ukrainian => "Ukrainian",
            Self::Dutch => "Dutch",
            Self::Thai => "Thai",
            Self::Greek => "Greek",
            Self::Hebrew => "Hebrew",
            Self::Swedish => "Swedish",
            Self::Indonesian => "Indonesian",
            Self::Czech => "Czech",
            Self::Finnish => "Finnish",
            Self::Danish => "Danish",
            Self::Norwegian => "Norwegian",
            Self::Hungarian => "Hungarian",
            Self::Persian => "Persian",
            Self::Swahili => "Swahili",
            Self::Romanian => "Romanian",
            Self::Tamil => "Tamil",
            Self::Telugu => "Telugu",
            Self::Armenian => "Armenian",
            Self::Georgian => "Georgian",
            Self::Yoruba => "Yoruba",
            Self::Zulu => "Zulu",
            Self::Esperanto => "Esperanto",
            Self::Lojban => "Lojban",
            Self::Klingon => "Klingon",
            Self::Latin => "Latin",
            Self::Sanskrit => "Sanskrit",
            Self::AncientGreek => "Ancient Greek",
            Self::AmericanSignLanguage => "American Sign Language",
            Self::BritishSignLanguage => "British Sign Language",
        }
    }
}

impl core::fmt::Display for HumanLanguageName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DialectName {
    #[default]
    AmericanEnglish,
    BritishEnglish,
    AustralianEnglish,
    EnglishKatakana,
    EnglishCyrillic,
    EnglishHangul,
    Mandarin,
    Cantonese,
    Hokkien,
    CastilianSpanish,
    MexicanSpanish,
    RioplatenseSpanish,
    ModernStandardHindi,
    ModernStandardArabic,
    EgyptianArabic,
    LevantineArabic,
    StandardBengali,
    BrazilianPortuguese,
    EuropeanPortuguese,
    StandardRussian,
    StandardJapanese,
    KansaiJapanese,
    StandardGerman,
    SwissGerman,
    Bavarian,
    MetropolitanFrench,
    QuebecoisFrench,
    StandardItalian,
    SouthKorean,
    NorthKorean,
    IstanbulTurkish,
    StandardVietnamese,
    StandardPolish,
    StandardUkrainian,
    StandardDutch,
    CentralThai,
    ModernGreek,
    ModernHebrew,
    StandardSwedish,
    StandardIndonesian,
    StandardCzech,
    StandardFinnish,
    StandardDanish,
    Bokmal,
    Nynorsk,
    StandardHungarian,
    IranianPersian,
    StandardSwahili,
    StandardRomanian,
    StandardTamil,
    StandardTelugu,
    EasternArmenian,
    WesternArmenian,
    StandardGeorgian,
    StandardYoruba,
    StandardZulu,
    StandardEsperanto,
    StandardLojban,
    StandardKlingon,
    ClassicalLatin,
    ClassicalSanskrit,
    AtticGreek,
    Asl,
    Bsl,
}

impl DialectName {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmericanEnglish => "American English",
            Self::BritishEnglish => "British English",
            Self::AustralianEnglish => "Australian English",
            Self::EnglishKatakana => "English (Katakana)",
            Self::EnglishCyrillic => "English (Cyrillic)",
            Self::EnglishHangul => "English (Hangul)",
            Self::Mandarin => "Mandarin",
            Self::Cantonese => "Cantonese",
            Self::Hokkien => "Hokkien",
            Self::CastilianSpanish => "Castilian Spanish",
            Self::MexicanSpanish => "Mexican Spanish",
            Self::RioplatenseSpanish => "Rioplatense Spanish",
            Self::ModernStandardHindi => "Modern Standard Hindi",
            Self::ModernStandardArabic => "Modern Standard Arabic",
            Self::EgyptianArabic => "Egyptian Arabic",
            Self::LevantineArabic => "Levantine Arabic",
            Self::StandardBengali => "Standard Bengali",
            Self::BrazilianPortuguese => "Brazilian Portuguese",
            Self::EuropeanPortuguese => "European Portuguese",
            Self::StandardRussian => "Standard Russian",
            Self::StandardJapanese => "Standard Japanese",
            Self::KansaiJapanese => "Kansai Japanese",
            Self::StandardGerman => "Standard German",
            Self::SwissGerman => "Swiss German",
            Self::Bavarian => "Bavarian",
            Self::MetropolitanFrench => "Metropolitan French",
            Self::QuebecoisFrench => "Québécois French",
            Self::StandardItalian => "Standard Italian",
            Self::SouthKorean => "South Korean",
            Self::NorthKorean => "North Korean",
            Self::IstanbulTurkish => "Istanbul Turkish",
            Self::StandardVietnamese => "Standard Vietnamese",
            Self::StandardPolish => "Standard Polish",
            Self::StandardUkrainian => "Standard Ukrainian",
            Self::StandardDutch => "Standard Dutch",
            Self::CentralThai => "Central Thai",
            Self::ModernGreek => "Modern Greek",
            Self::ModernHebrew => "Modern Hebrew",
            Self::StandardSwedish => "Standard Swedish",
            Self::StandardIndonesian => "Standard Indonesian",
            Self::StandardCzech => "Standard Czech",
            Self::StandardFinnish => "Standard Finnish",
            Self::StandardDanish => "Standard Danish",
            Self::Bokmal => "Bokmål",
            Self::Nynorsk => "Nynorsk",
            Self::StandardHungarian => "Standard Hungarian",
            Self::IranianPersian => "Iranian Persian",
            Self::StandardSwahili => "Standard Swahili",
            Self::StandardRomanian => "Standard Romanian",
            Self::StandardTamil => "Standard Tamil",
            Self::StandardTelugu => "Standard Telugu",
            Self::EasternArmenian => "Eastern Armenian",
            Self::WesternArmenian => "Western Armenian",
            Self::StandardGeorgian => "Standard Georgian",
            Self::StandardYoruba => "Standard Yoruba",
            Self::StandardZulu => "Standard Zulu",
            Self::StandardEsperanto => "Standard Esperanto",
            Self::StandardLojban => "Standard Lojban",
            Self::StandardKlingon => "Standard Klingon",
            Self::ClassicalLatin => "Classical Latin",
            Self::ClassicalSanskrit => "Classical Sanskrit",
            Self::AtticGreek => "Attic Greek",
            Self::Asl => "ASL",
            Self::Bsl => "BSL",
        }
    }
}

impl core::fmt::Display for DialectName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HumanLanguage {
    pub name: HumanLanguageName,
    pub categories: &'static [HumanLanguageCategory],
    pub scripts: &'static [ScriptName], // scripts used by the language overall
    pub dialects: &'static [Dialect],
}

#[derive(Debug, Clone, Copy)]
pub struct Dialect {
    pub label: DialectName,            // "Mandarin", "Cantonese", "ASL" vs "BSL"
    pub script: &'static [ScriptName], // this dialect's scripts; empty if no written form
    pub hello_worlds: &'static [&'static str],
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
        ],
        dialects: &[
            Dialect {
                label: DialectName::AmericanEnglish,
                script: &[ScriptName::Latin],
                hello_worlds: &["Hello, World!"],
            },
            Dialect {
                label: DialectName::BritishEnglish,
                script: &[ScriptName::Latin],
                hello_worlds: &["Hello, World!"],
            },
            Dialect {
                label: DialectName::AustralianEnglish,
                script: &[ScriptName::Latin],
                hello_worlds: &["G'day, World!"],
            },
            Dialect {
                label: DialectName::EnglishKatakana,
                script: &[ScriptName::Katakana],
                hello_worlds: &["ハローワールド", "ハーロ、ワルド"],
            },
            Dialect {
                label: DialectName::EnglishCyrillic,
                script: &[ScriptName::Cyrillic],
                hello_worlds: &["Хелло, Ворлд"],
            },
            Dialect {
                label: DialectName::EnglishHangul,
                script: &[ScriptName::Hangul],
                hello_worlds: &["헬로, 월드!"],
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
        ],
        dialects: &[
            Dialect {
                label: DialectName::Mandarin,
                script: &[ScriptName::SimplifiedChinese],
                hello_worlds: &["你好，世界！"],
            },
            Dialect {
                label: DialectName::Cantonese,
                script: &[ScriptName::TraditionalChinese],
                hello_worlds: &["哈囉，世界！"],
            },
            Dialect {
                label: DialectName::Hokkien,
                script: &[ScriptName::TraditionalChinese],
                hello_worlds: &["汝好，世界！"],
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
                script: &[ScriptName::Latin],
                hello_worlds: &["¡Hola, Mundo!"],
            },
            Dialect {
                label: DialectName::MexicanSpanish,
                script: &[ScriptName::Latin],
                hello_worlds: &["¡Hola, Mundo!"],
            },
            Dialect {
                label: DialectName::RioplatenseSpanish,
                script: &[ScriptName::Latin],
                hello_worlds: &["¡Hola, Mundo!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Hindi,
        categories: &[Natural],
        scripts: &[ScriptName::Devanagari],
        dialects: &[Dialect {
            label: DialectName::ModernStandardHindi,
            script: &[ScriptName::Devanagari],
            hello_worlds: &["नमस्ते दुनिया!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Arabic,
        categories: &[Natural],
        scripts: &[ScriptName::Arabic],
        dialects: &[
            Dialect {
                label: DialectName::ModernStandardArabic,
                script: &[ScriptName::Arabic],
                hello_worlds: &["مرحبا بالعالم!"],
            },
            Dialect {
                label: DialectName::EgyptianArabic,
                script: &[ScriptName::Arabic],
                hello_worlds: &["أهلاً بالعالم!"],
            },
            Dialect {
                label: DialectName::LevantineArabic,
                script: &[ScriptName::Arabic],
                hello_worlds: &["مرحبا بالعالم!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Bengali,
        categories: &[Natural],
        scripts: &[ScriptName::Bengali],
        dialects: &[Dialect {
            label: DialectName::StandardBengali,
            script: &[ScriptName::Bengali],
            hello_worlds: &["হ্যালো, বিশ্ব!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Portuguese,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::BrazilianPortuguese,
                script: &[ScriptName::Latin],
                hello_worlds: &["Olá, Mundo!"],
            },
            Dialect {
                label: DialectName::EuropeanPortuguese,
                script: &[ScriptName::Latin],
                hello_worlds: &["Olá, Mundo!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Russian,
        categories: &[Natural],
        scripts: &[ScriptName::Cyrillic],
        dialects: &[Dialect {
            label: DialectName::StandardRussian,
            script: &[ScriptName::Cyrillic],
            hello_worlds: &["Привет, мир!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Japanese,
        categories: &[Natural],
        scripts: &[
            ScriptName::Kanji,
            ScriptName::Hiragana,
            ScriptName::Katakana,
        ],
        dialects: &[
            Dialect {
                label: DialectName::StandardJapanese,
                script: &[ScriptName::Kanji, ScriptName::Hiragana],
                hello_worlds: &["こんにちは世界！"],
            },
            Dialect {
                label: DialectName::KansaiJapanese,
                script: &[ScriptName::Kanji, ScriptName::Hiragana],
                hello_worlds: &["こんにちは世界！", "まいど、世界！"],
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
                script: &[ScriptName::Latin],
                hello_worlds: &["Hallo, Welt!"],
            },
            Dialect {
                label: DialectName::SwissGerman,
                script: &[ScriptName::Latin],
                hello_worlds: &["Hallo, Wält!"],
            },
            Dialect {
                label: DialectName::Bavarian,
                script: &[ScriptName::Latin],
                hello_worlds: &["Servus, Wöd!"],
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
                script: &[ScriptName::Latin],
                hello_worlds: &["Bonjour le monde!"],
            },
            Dialect {
                label: DialectName::QuebecoisFrench,
                script: &[ScriptName::Latin],
                hello_worlds: &["Bonjour le monde!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Italian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardItalian,
            script: &[ScriptName::Latin],
            hello_worlds: &["Ciao, mondo!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Korean,
        categories: &[Natural],
        scripts: &[ScriptName::Hangul],
        dialects: &[
            Dialect {
                label: DialectName::SouthKorean,
                script: &[ScriptName::Hangul],
                hello_worlds: &["안녕하세요, 세계!"],
            },
            Dialect {
                label: DialectName::NorthKorean,
                script: &[ScriptName::Hangul],
                hello_worlds: &["안녕하세요, 세계!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Turkish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::IstanbulTurkish,
            script: &[ScriptName::Latin],
            hello_worlds: &["Merhaba, Dünya!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Vietnamese,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardVietnamese,
            script: &[ScriptName::Latin],
            hello_worlds: &["Xin chào, thế giới!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Polish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardPolish,
            script: &[ScriptName::Latin],
            hello_worlds: &["Witaj, świecie!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Ukrainian,
        categories: &[Natural],
        scripts: &[ScriptName::Cyrillic],
        dialects: &[Dialect {
            label: DialectName::StandardUkrainian,
            script: &[ScriptName::Cyrillic],
            hello_worlds: &["Привіт, світе!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Dutch,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardDutch,
            script: &[ScriptName::Latin],
            hello_worlds: &["Hallo, wereld!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Thai,
        categories: &[Natural],
        scripts: &[ScriptName::Thai],
        dialects: &[Dialect {
            label: DialectName::CentralThai,
            script: &[ScriptName::Thai],
            hello_worlds: &["สวัสดีชาวโลก!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Greek,
        categories: &[Natural],
        scripts: &[ScriptName::Greek],
        dialects: &[Dialect {
            label: DialectName::ModernGreek,
            script: &[ScriptName::Greek],
            hello_worlds: &["Γεια σου, Κόσμε!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Hebrew,
        categories: &[Natural],
        scripts: &[ScriptName::Hebrew],
        dialects: &[Dialect {
            label: DialectName::ModernHebrew,
            script: &[ScriptName::Hebrew],
            hello_worlds: &["שלום, עולם!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Swedish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardSwedish,
            script: &[ScriptName::Latin],
            hello_worlds: &["Hej, världen!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Indonesian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardIndonesian,
            script: &[ScriptName::Latin],
            hello_worlds: &["Halo, Dunia!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Czech,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardCzech,
            script: &[ScriptName::Latin],
            hello_worlds: &["Ahoj, světe!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Finnish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardFinnish,
            script: &[ScriptName::Latin],
            hello_worlds: &["Hei, maailma!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Danish,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardDanish,
            script: &[ScriptName::Latin],
            hello_worlds: &["Hej, verden!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Norwegian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[
            Dialect {
                label: DialectName::Bokmal,
                script: &[ScriptName::Latin],
                hello_worlds: &["Hei, verden!"],
            },
            Dialect {
                label: DialectName::Nynorsk,
                script: &[ScriptName::Latin],
                hello_worlds: &["Hei, verda!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Hungarian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardHungarian,
            script: &[ScriptName::Latin],
            hello_worlds: &["Helló, világ!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Persian,
        categories: &[Natural],
        scripts: &[ScriptName::PersoArabic],
        dialects: &[Dialect {
            label: DialectName::IranianPersian,
            script: &[ScriptName::PersoArabic],
            hello_worlds: &["سلام دنیا!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Swahili,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardSwahili,
            script: &[ScriptName::Latin],
            hello_worlds: &["Habari, Dunia!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Romanian,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardRomanian,
            script: &[ScriptName::Latin],
            hello_worlds: &["Salut, lume!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Tamil,
        categories: &[Natural],
        scripts: &[ScriptName::Tamil],
        dialects: &[Dialect {
            label: DialectName::StandardTamil,
            script: &[ScriptName::Tamil],
            hello_worlds: &["வணக்கம், உலகம்!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Telugu,
        categories: &[Natural],
        scripts: &[ScriptName::Telugu],
        dialects: &[Dialect {
            label: DialectName::StandardTelugu,
            script: &[ScriptName::Telugu],
            hello_worlds: &["హలో, ప్రపంచం!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Armenian,
        categories: &[Natural],
        scripts: &[ScriptName::Armenian],
        dialects: &[
            Dialect {
                label: DialectName::EasternArmenian,
                script: &[ScriptName::Armenian],
                hello_worlds: &["Բարեւ, աշխարհ!"],
            },
            Dialect {
                label: DialectName::WesternArmenian,
                script: &[ScriptName::Armenian],
                hello_worlds: &["Բարեւ աշխարհ!"],
            },
        ],
    },
    HumanLanguage {
        name: HumanLanguageName::Georgian,
        categories: &[Natural],
        scripts: &[ScriptName::Georgian],
        dialects: &[Dialect {
            label: DialectName::StandardGeorgian,
            script: &[ScriptName::Georgian],
            hello_worlds: &["გამარჯობა, მსოფლიო!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Yoruba,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardYoruba,
            script: &[ScriptName::Latin],
            hello_worlds: &["Ẹ n lẹ, Ayé!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Zulu,
        categories: &[Natural],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardZulu,
            script: &[ScriptName::Latin],
            hello_worlds: &["Sawubona, Mhlaba!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Esperanto,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardEsperanto,
            script: &[ScriptName::Latin],
            hello_worlds: &["Saluton, mondo!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Lojban,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::StandardLojban,
            script: &[ScriptName::Latin],
            hello_worlds: &["coi munje"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Klingon,
        categories: &[Constructed],
        scripts: &[ScriptName::Latin, ScriptName::Piqad],
        dialects: &[Dialect {
            label: DialectName::StandardKlingon,
            script: &[ScriptName::Latin],
            hello_worlds: &["nuqneH, qo'!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Latin,
        categories: &[Extinct],
        scripts: &[ScriptName::Latin],
        dialects: &[Dialect {
            label: DialectName::ClassicalLatin,
            script: &[ScriptName::Latin],
            hello_worlds: &["Salve, munde!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::Sanskrit,
        categories: &[Natural],
        scripts: &[ScriptName::Devanagari],
        dialects: &[Dialect {
            label: DialectName::ClassicalSanskrit,
            script: &[ScriptName::Devanagari],
            hello_worlds: &["नमस्ते विश्व!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::AncientGreek,
        categories: &[Extinct],
        scripts: &[ScriptName::Greek],
        dialects: &[Dialect {
            label: DialectName::AtticGreek,
            script: &[ScriptName::Greek],
            hello_worlds: &["Χαῖρε, κόσμε!"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::AmericanSignLanguage,
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: DialectName::Asl,
            script: &[],
            hello_worlds: &["HELLO WORLD"],
        }],
    },
    HumanLanguage {
        name: HumanLanguageName::BritishSignLanguage,
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: DialectName::Bsl,
            script: &[],
            hello_worlds: &["HELLO WORLD"],
        }],
    },
];

/// Errors that can occur while looking up localized greetings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum HumanLanguageError {
    /// The language is not recorded in the dataset.
    #[error("no greetings found for language {0}: not a recorded language")]
    UnknownLanguage(HumanLanguageName),

    /// The language is recorded, but the requested dialect does not belong to it.
    #[error(
        "no greetings found for {language}: dialect {dialect} is not recorded for this language"
    )]
    UnknownDialect {
        /// The requested language.
        language: HumanLanguageName,
        /// The requested dialect.
        dialect: DialectName,
    },

    /// The dialect has no recorded greetings.
    #[error("no greeting recorded for {language} ({dialect})")]
    NoGreeting {
        /// The requested language.
        language: HumanLanguageName,
        /// The requested dialect.
        dialect: DialectName,
    },
}

/// Look up the canonical (first) localized greeting for a language and dialect.
///
/// # Errors
///
/// Returns a [`HumanLanguageError`] if the language is not recorded, if the
/// dialect is not one of the language's dialects, or if the dialect has no
/// recorded greeting.
pub fn greeting_for(
    language: HumanLanguageName,
    dialect: DialectName,
) -> Result<&'static str, HumanLanguageError> {
    let lang = HUMAN_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(HumanLanguageError::UnknownLanguage(language))?;
    let lang_dialect = lang
        .dialects
        .iter()
        .find(|d| d.label == dialect)
        .ok_or(HumanLanguageError::UnknownDialect { language, dialect })?;
    lang_dialect
        .hello_worlds
        .first()
        .copied()
        .ok_or(HumanLanguageError::NoGreeting { language, dialect })
}

/// Look up all localized greetings for a language and dialect.
///
/// # Errors
///
/// Returns a [`HumanLanguageError`] if the language is not recorded, or if the
/// dialect is not one of the language's dialects.
pub fn greetings_for(
    language: HumanLanguageName,
    dialect: DialectName,
) -> Result<&'static [&'static str], HumanLanguageError> {
    let lang = HUMAN_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(HumanLanguageError::UnknownLanguage(language))?;
    lang.dialects
        .iter()
        .find(|d| d.label == dialect)
        .map(|d| d.hello_worlds)
        .ok_or(HumanLanguageError::UnknownDialect { language, dialect })
}
