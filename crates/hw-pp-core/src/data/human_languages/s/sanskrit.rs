//! Hello-world greetings for `HumanLanguageName::Sanskrit`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sanskrit => "Sanskrit",
    dialects: [ClassicalSanskrit => "Classical Sanskrit"],
    scripts: [Devanagari => "Devanagari"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sanskrit,
    categories: &[Natural],
    scripts: &[ScriptName::Devanagari],
    dialects: &[Dialect {
        label: DialectName::ClassicalSanskrit,
        greetings: &[Greeting {
            scripts: &[ScriptName::Devanagari],
            text: "नमस्ते विश्व!",
        }],
    }],
};
