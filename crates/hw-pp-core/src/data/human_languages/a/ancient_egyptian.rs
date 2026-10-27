//! Hello-world greetings for `HumanLanguageName::AncientEgyptian`.
//!
//! `j.nḏ ḥr.k` ("hail to you", with the masculine suffix `.k`, as `tꜣ` is masculine) addresses
//! `tꜣ` ("the land"); both words are attested, the phrase itself is composed from them. The
//! hieroglyphs follow Wiktionary's spellings, written as a plain linear sign sequence without
//! quadrat format controls (Gardiner M17, Aa27, D46, D2, D21, V31; N17, N21, Z1).
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/j.n%E1%B8%8F_%E1%B8%A5r#Egyptian>
//! - <https://en.wiktionary.org/wiki/t%EA%9C%A3#Egyptian>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Extinct,
    HumanLanguageFamily::{Afroasiatic, Egyptian},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: AncientEgyptian => "Ancient Egyptian",
    dialects: [MiddleEgyptian => "Middle Egyptian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::AncientEgyptian,
    categories: &[Extinct],
    families: &[Afroasiatic, Egyptian],
    scripts: &[ScriptName::EgyptianHieroglyphs, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::MiddleEgyptian,
        greetings: &[
            // Hieroglyphs have no punctuation.
            Greeting {
                scripts: &[ScriptName::EgyptianHieroglyphs],
                text: "𓇋𓐩𓂧𓁷𓂋𓎡 𓇿𓈅𓏤",
            },
            // Egyptological transliteration; `.` separates morphemes.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "j.nḏ ḥr.k, tꜣ!",
            },
        ],
    }],
};
