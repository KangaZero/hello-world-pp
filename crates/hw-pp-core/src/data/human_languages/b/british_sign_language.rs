//! Hello-world greetings for `HumanLanguageName::BritishSignLanguage`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Signed, HumanLanguageName,
};
use crate::data::macros::language_names;

language_names! {
    language: BritishSignLanguage => "British Sign Language",
    dialects: [Bsl => "BSL"],
    scripts: [],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
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
};
