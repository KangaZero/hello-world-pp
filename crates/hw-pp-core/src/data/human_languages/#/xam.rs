//! Hello-world greetings for `HumanLanguageName::Xam`.
//!
//! The endonym is ǀXam-ka ǃei. Bleek's ǃãũ is written ǃa͠u (U+0360 double tilde), as on Wiktionary.
//!
//! Sources:
//! - <https://archive.org/details/bushmandictionar00blee>
//! - <https://en.wikipedia.org/wiki/%C7%80Xam_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Extinct, Natural},
    HumanLanguageFamily::Tuu,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Xam => "ǀXam" | "Xam",
    dialects: [StandardXam => "Standard ǀXam"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Xam,
    categories: &[Natural, Extinct],
    families: &[Tuu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardXam,
        greetings: &[
            // oe "hi, ho there" (an attention call, not a formal greeting) + ǃa͠u "earth, ground"
            // (Bleek 1956, pp. 153, 372); composed from attested words, the phrase itself is unattested.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Oe, ǃa͠u!",
            },
        ],
    }],
};
