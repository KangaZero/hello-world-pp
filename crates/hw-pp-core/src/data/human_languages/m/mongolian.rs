//! Hello-world greetings for `HumanLanguageName::Mongolian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B0%D0%B9%D0%BD#Mongolian>
//! - <https://en.wiktionary.org/wiki/%D1%83%D1%83#Mongolian>
//! - <https://en.wiktionary.org/wiki/%E1%A0%A4%E1%A0%A4#Mongolian>
//! - <https://en.wiktionary.org/wiki/%D0%B4%D1%8D%D0%BB%D1%85%D0%B8%D0%B9#Mongolian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Mongolic, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Mongolian => "Mongolian" | "Монгол хэл",
    dialects: [KhalkhaMongolian => "Khalkha Mongolian" | "Халх монгол"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Mongolian,
    categories: &[Natural],
    families: &[Mongolic],
    scripts: &[
        ScriptName::Cyrillic,
        ScriptName::Mongolian,
        ScriptName::Latin,
    ],
    dialects: &[Dialect {
        label: DialectName::KhalkhaMongolian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Сайн уу, дэлхий!",
            },
            Greeting {
                scripts: &[ScriptName::Mongolian],
                text: "ᠰᠠᠶᠢᠨ ᠤᠤ ᠳᠡᠯᠡᠭᠡᠢ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Sain uu, delkhii!",
            },
        ],
    }],
};
