//! Hello-world greetings for `HumanLanguageName::AmericanSignLanguage`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Signed, HumanLanguageName,
};
use crate::data::macros::language_names;

language_names! {
    language: AmericanSignLanguage => "American Sign Language",
    dialects: [Asl => "ASL"],
    scripts: [],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::AmericanSignLanguage,
    categories: &[Signed],
    scripts: &[],
    dialects: &[Dialect {
        label: DialectName::Asl,
        greetings: &[Greeting {
            scripts: &[],
            text: "HELLO WORLD",
        }],
    }],
};
