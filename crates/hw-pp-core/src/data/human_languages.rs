use super::common::HumanLanguageCategory;
use HumanLanguageCategory::{Constructed, Extinct, Natural, Signed};

#[derive(Debug, Clone, Copy)]
pub struct HumanLanguage {
    pub name: &'static str,
    pub categories: &'static [HumanLanguageCategory],
    pub scripts: &'static [&'static str], // scripts used by the language overall
    pub dialects: &'static [Dialect],
}

#[derive(Debug, Clone, Copy)]
pub struct Dialect {
    pub label: &'static str,          // "Mandarin", "Cantonese", "ASL" vs "BSL"
    pub script: Option<&'static str>, // this dialect's script, if it has a written form
    pub hello_world: &'static str,
}

pub static HUMAN_LANGUAGES: &[HumanLanguage] = &[
    HumanLanguage {
        name: "English",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "American English",
                script: Some("Latin"),
                hello_world: "Hello, World!",
            },
            Dialect {
                label: "British English",
                script: Some("Latin"),
                hello_world: "Hello, World!",
            },
            Dialect {
                label: "Australian English",
                script: Some("Latin"),
                hello_world: "G'day, World!",
            },
        ],
    },
    HumanLanguage {
        name: "Chinese",
        categories: &[Natural],
        scripts: &["Han", "Simplified Chinese", "Traditional Chinese"],
        dialects: &[
            Dialect {
                label: "Mandarin",
                script: Some("Simplified Chinese"),
                hello_world: "你好，世界！",
            },
            Dialect {
                label: "Cantonese",
                script: Some("Traditional Chinese"),
                hello_world: "哈囉，世界！",
            },
            Dialect {
                label: "Hokkien",
                script: Some("Traditional Chinese"),
                hello_world: "汝好，世界！",
            },
        ],
    },
    HumanLanguage {
        name: "Spanish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "Castilian Spanish",
                script: Some("Latin"),
                hello_world: "¡Hola, Mundo!",
            },
            Dialect {
                label: "Mexican Spanish",
                script: Some("Latin"),
                hello_world: "¡Hola, Mundo!",
            },
            Dialect {
                label: "Rioplatense Spanish",
                script: Some("Latin"),
                hello_world: "¡Hola, Mundo!",
            },
        ],
    },
    HumanLanguage {
        name: "Hindi",
        categories: &[Natural],
        scripts: &["Devanagari"],
        dialects: &[Dialect {
            label: "Modern Standard Hindi",
            script: Some("Devanagari"),
            hello_world: "नमस्ते दुनिया!",
        }],
    },
    HumanLanguage {
        name: "Arabic",
        categories: &[Natural],
        scripts: &["Arabic"],
        dialects: &[
            Dialect {
                label: "Modern Standard Arabic",
                script: Some("Arabic"),
                hello_world: "مرحبا بالعالم!",
            },
            Dialect {
                label: "Egyptian Arabic",
                script: Some("Arabic"),
                hello_world: "أهلاً بالعالم!",
            },
            Dialect {
                label: "Levantine Arabic",
                script: Some("Arabic"),
                hello_world: "مرحبا بالعالم!",
            },
        ],
    },
    HumanLanguage {
        name: "Bengali",
        categories: &[Natural],
        scripts: &["Bengali"],
        dialects: &[Dialect {
            label: "Standard Bengali",
            script: Some("Bengali"),
            hello_world: "হ্যালো, বিশ্ব!",
        }],
    },
    HumanLanguage {
        name: "Portuguese",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "Brazilian Portuguese",
                script: Some("Latin"),
                hello_world: "Olá, Mundo!",
            },
            Dialect {
                label: "European Portuguese",
                script: Some("Latin"),
                hello_world: "Olá, Mundo!",
            },
        ],
    },
    HumanLanguage {
        name: "Russian",
        categories: &[Natural],
        scripts: &["Cyrillic"],
        dialects: &[Dialect {
            label: "Standard Russian",
            script: Some("Cyrillic"),
            hello_world: "Привет, мир!",
        }],
    },
    HumanLanguage {
        name: "Japanese",
        categories: &[Natural],
        scripts: &["Kanji", "Hiragana", "Katakana"],
        dialects: &[
            Dialect {
                label: "Standard Japanese",
                script: Some("Kanji, Hiragana"),
                hello_world: "こんにちは世界！",
            },
            Dialect {
                label: "Kansai Japanese",
                script: Some("Kanji, Hiragana"),
                hello_world: "こんにちは世界！",
            },
        ],
    },
    HumanLanguage {
        name: "German",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "Standard German",
                script: Some("Latin"),
                hello_world: "Hallo, Welt!",
            },
            Dialect {
                label: "Swiss German",
                script: Some("Latin"),
                hello_world: "Hallo, Wält!",
            },
            Dialect {
                label: "Bavarian",
                script: Some("Latin"),
                hello_world: "Servus, Wöd!",
            },
        ],
    },
    HumanLanguage {
        name: "French",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "Metropolitan French",
                script: Some("Latin"),
                hello_world: "Bonjour le monde!",
            },
            Dialect {
                label: "Québécois French",
                script: Some("Latin"),
                hello_world: "Bonjour le monde!",
            },
        ],
    },
    HumanLanguage {
        name: "Italian",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Italian",
            script: Some("Latin"),
            hello_world: "Ciao, mondo!",
        }],
    },
    HumanLanguage {
        name: "Korean",
        categories: &[Natural],
        scripts: &["Hangul"],
        dialects: &[
            Dialect {
                label: "South Korean",
                script: Some("Hangul"),
                hello_world: "안녕하세요, 세계!",
            },
            Dialect {
                label: "North Korean",
                script: Some("Hangul"),
                hello_world: "안녕하세요, 세계!",
            },
        ],
    },
    HumanLanguage {
        name: "Turkish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Istanbul Turkish",
            script: Some("Latin"),
            hello_world: "Merhaba, Dünya!",
        }],
    },
    HumanLanguage {
        name: "Vietnamese",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Vietnamese",
            script: Some("Latin"),
            hello_world: "Xin chào, thế giới!",
        }],
    },
    HumanLanguage {
        name: "Polish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Polish",
            script: Some("Latin"),
            hello_world: "Witaj, świecie!",
        }],
    },
    HumanLanguage {
        name: "Ukrainian",
        categories: &[Natural],
        scripts: &["Cyrillic"],
        dialects: &[Dialect {
            label: "Standard Ukrainian",
            script: Some("Cyrillic"),
            hello_world: "Привіт, світе!",
        }],
    },
    HumanLanguage {
        name: "Dutch",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Dutch",
            script: Some("Latin"),
            hello_world: "Hallo, wereld!",
        }],
    },
    HumanLanguage {
        name: "Thai",
        categories: &[Natural],
        scripts: &["Thai"],
        dialects: &[Dialect {
            label: "Central Thai",
            script: Some("Thai"),
            hello_world: "สวัสดีชาวโลก!",
        }],
    },
    HumanLanguage {
        name: "Greek",
        categories: &[Natural],
        scripts: &["Greek"],
        dialects: &[Dialect {
            label: "Modern Greek",
            script: Some("Greek"),
            hello_world: "Γεια σου, Κόσμε!",
        }],
    },
    HumanLanguage {
        name: "Hebrew",
        categories: &[Natural],
        scripts: &["Hebrew"],
        dialects: &[Dialect {
            label: "Modern Hebrew",
            script: Some("Hebrew"),
            hello_world: "שלום, עולם!",
        }],
    },
    HumanLanguage {
        name: "Swedish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Swedish",
            script: Some("Latin"),
            hello_world: "Hej, världen!",
        }],
    },
    HumanLanguage {
        name: "Indonesian",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Indonesian",
            script: Some("Latin"),
            hello_world: "Halo, Dunia!",
        }],
    },
    HumanLanguage {
        name: "Czech",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Czech",
            script: Some("Latin"),
            hello_world: "Ahoj, světe!",
        }],
    },
    HumanLanguage {
        name: "Finnish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Finnish",
            script: Some("Latin"),
            hello_world: "Hei, maailma!",
        }],
    },
    HumanLanguage {
        name: "Danish",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Danish",
            script: Some("Latin"),
            hello_world: "Hej, verden!",
        }],
    },
    HumanLanguage {
        name: "Norwegian",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[
            Dialect {
                label: "Bokmål",
                script: Some("Latin"),
                hello_world: "Hei, verden!",
            },
            Dialect {
                label: "Nynorsk",
                script: Some("Latin"),
                hello_world: "Hei, verda!",
            },
        ],
    },
    HumanLanguage {
        name: "Hungarian",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Hungarian",
            script: Some("Latin"),
            hello_world: "Helló, világ!",
        }],
    },
    HumanLanguage {
        name: "Persian",
        categories: &[Natural],
        scripts: &["Perso-Arabic"],
        dialects: &[Dialect {
            label: "Iranian Persian",
            script: Some("Perso-Arabic"),
            hello_world: "سلام دنیا!",
        }],
    },
    HumanLanguage {
        name: "Swahili",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Swahili",
            script: Some("Latin"),
            hello_world: "Habari, Dunia!",
        }],
    },
    HumanLanguage {
        name: "Romanian",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Romanian",
            script: Some("Latin"),
            hello_world: "Salut, lume!",
        }],
    },
    HumanLanguage {
        name: "Tamil",
        categories: &[Natural],
        scripts: &["Tamil"],
        dialects: &[Dialect {
            label: "Standard Tamil",
            script: Some("Tamil"),
            hello_world: "வணக்கம், உலகம்!",
        }],
    },
    HumanLanguage {
        name: "Telugu",
        categories: &[Natural],
        scripts: &["Telugu"],
        dialects: &[Dialect {
            label: "Standard Telugu",
            script: Some("Telugu"),
            hello_world: "హలో, ప్రపంచం!",
        }],
    },
    HumanLanguage {
        name: "Armenian",
        categories: &[Natural],
        scripts: &["Armenian"],
        dialects: &[
            Dialect {
                label: "Eastern Armenian",
                script: Some("Armenian"),
                hello_world: "Բարեւ, աշխարհ!",
            },
            Dialect {
                label: "Western Armenian",
                script: Some("Armenian"),
                hello_world: "Բարեւ աշխարհ!",
            },
        ],
    },
    HumanLanguage {
        name: "Georgian",
        categories: &[Natural],
        scripts: &["Georgian"],
        dialects: &[Dialect {
            label: "Standard Georgian",
            script: Some("Georgian"),
            hello_world: "გამარჯობა, მსოფლიო!",
        }],
    },
    HumanLanguage {
        name: "Yoruba",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Yoruba",
            script: Some("Latin"),
            hello_world: "Ẹ n lẹ, Ayé!",
        }],
    },
    HumanLanguage {
        name: "Zulu",
        categories: &[Natural],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Zulu",
            script: Some("Latin"),
            hello_world: "Sawubona, Mhlaba!",
        }],
    },
    HumanLanguage {
        name: "Esperanto",
        categories: &[Constructed],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Esperanto",
            script: Some("Latin"),
            hello_world: "Saluton, mondo!",
        }],
    },
    HumanLanguage {
        name: "Lojban",
        categories: &[Constructed],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Standard Lojban",
            script: Some("Latin"),
            hello_world: "coi munje",
        }],
    },
    HumanLanguage {
        name: "Klingon",
        categories: &[Constructed],
        scripts: &["Latin", "pIqaD"],
        dialects: &[Dialect {
            label: "Standard Klingon",
            script: Some("Latin"),
            hello_world: "nuqneH, qo'!",
        }],
    },
    HumanLanguage {
        name: "Latin",
        categories: &[Extinct],
        scripts: &["Latin"],
        dialects: &[Dialect {
            label: "Classical Latin",
            script: Some("Latin"),
            hello_world: "Salve, munde!",
        }],
    },
    HumanLanguage {
        name: "Sanskrit",
        categories: &[Natural],
        scripts: &["Devanagari"],
        dialects: &[Dialect {
            label: "Classical Sanskrit",
            script: Some("Devanagari"),
            hello_world: "नमस्ते विश्व!",
        }],
    },
    HumanLanguage {
        name: "Ancient Greek",
        categories: &[Extinct],
        scripts: &["Greek"],
        dialects: &[Dialect {
            label: "Attic Greek",
            script: Some("Greek"),
            hello_world: "Χαῖρε, κόσμε!",
        }],
    },
    HumanLanguage {
        name: "American Sign Language",
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: "ASL",
            script: None,
            hello_world: "HELLO WORLD",
        }],
    },
    HumanLanguage {
        name: "British Sign Language",
        categories: &[Signed],
        scripts: &[],
        dialects: &[Dialect {
            label: "BSL",
            script: None,
            hello_world: "HELLO WORLD",
        }],
    },
];
