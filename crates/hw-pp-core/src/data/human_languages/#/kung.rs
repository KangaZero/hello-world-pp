//! Hello-world greetings for `HumanLanguageName::Kung`.
//!
//! ǃKung is a dialect cluster; only Juǀʼhoan, its best-documented variety, is recorded.
//!
//! Sources:
//! - <https://web.archive.org/web/20111028045804/http://kalaharipeoples.org/academic/dickens/ju_lessons/ju_lessons_text.pdf>
//! - <https://en.wikipedia.org/wiki/Ju%C7%80%CA%BChoan_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Kxa, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Kung => "ǃKung" | "Kung",
    dialects: [Juhoan => "Juǀʼhoan" | "Jùǀʼhoànsi"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kung,
    categories: &[Natural],
    families: &[Kxa],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::Juhoan,
        greetings: &[
            // ǂXáí-ó "hello" (morning; Gǁàán-ó in the afternoon) + kxà "earth" (Dickens & Biesele
            // 2009, lesson one and idioms); composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "ǂXáí-ó, kxà!",
            },
        ],
    }],
};
