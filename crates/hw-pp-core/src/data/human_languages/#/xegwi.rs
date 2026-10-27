//! Hello-world greetings for `HumanLanguageName::Xegwi`.
//!
//! The endonym is giǁkxi꞉gwi.
//!
//! Sources:
//! - <https://archive.org/details/bushmandictionar00blee>
//! - <https://en.wikipedia.org/wiki/%C7%81Xegwi_language>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Extinct, Natural},
    HumanLanguageFamily::Tuu,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Xegwi => "ǁXegwi" | "Xegwi",
    dialects: [StandardXegwi => "Standard ǁXegwi"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Xegwi,
    categories: &[Natural, Extinct],
    families: &[Tuu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardXegwi,
        greetings: &[
            // oa to "a greeting (?) good day" (Bleek marks the gloss uncertain) + ǀkama "open
            // country, veld" (Bleek 1956, pp. 153, 299, her SIII = Lake Chrissie); composed from attested words.
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Oa to, ǀkama!",
            },
        ],
    }],
};
