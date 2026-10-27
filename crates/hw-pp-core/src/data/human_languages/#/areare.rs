//! Hello-world greetings for `HumanLanguageName::Areare`.
//!
//! The glottal stop is written Ꞌ/ꞌ (U+A78B/U+A78C) as on Wikipedia; sources also use ' and ɂ.
//!
//! Sources:
//! - <https://www.bible.com/bible/3761/MRK.15.18.ALU>
//! - <https://www.bible.com/bible/3761/1JN.2.15.ALU>
//! - <https://en.wikipedia.org/wiki/%EA%9E%8BAre%EA%9E%8Care_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austronesian, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Areare => "ꞋAreꞌare" | "Areare",
    dialects: [StandardAreare => "Standard ꞋAreꞌare"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Areare,
    categories: &[Natural],
    families: &[Austronesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAreare,
        greetings: &[
            // raemanoha "hail" (Mark 15:18) + maraukari "world" (1 John 2:15, 4:9; Mark 16:15);
            // composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Raemanoha, maraukari!",
            },
        ],
    }],
};
