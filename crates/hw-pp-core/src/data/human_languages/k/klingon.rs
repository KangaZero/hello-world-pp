//! Hello-world greetings for `HumanLanguageName::Klingon`.
//!
//! Sources:
//! - <https://www.kli.org/about-klingon/klingon-phrases/>
//! - <https://en.wikipedia.org/wiki/Klingon_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Constructed, Fictional},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Klingon => "Klingon" | "tlhIngan Hol",
    dialects: [StandardKlingon => "Standard Klingon"],
    scripts: [Latin => "Latin", Piqad => "pIqaD"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Klingon,
    categories: &[Constructed, Fictional],
    scripts: &[ScriptName::Latin, ScriptName::Piqad],
    dialects: &[Dialect {
        label: DialectName::StandardKlingon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "nuqneH, qo'!",
        }],
    }],
};
