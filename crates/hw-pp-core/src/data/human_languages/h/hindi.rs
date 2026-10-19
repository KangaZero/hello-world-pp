//! Hello-world greetings for `HumanLanguageName::Hindi`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hindi => "Hindi" | "हिन्दी",
    dialects: [ModernStandardHindi => "Modern Standard Hindi"],
    scripts: [Devanagari => "Devanagari"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hindi,
    categories: &[Natural],
    scripts: &[ScriptName::Devanagari],
    dialects: &[Dialect {
        label: DialectName::ModernStandardHindi,
        greetings: &[Greeting {
            scripts: &[ScriptName::Devanagari],
            text: "नमस्ते दुनिया!",
        }],
    }],
};
