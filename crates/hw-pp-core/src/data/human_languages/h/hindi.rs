//! Hello-world greetings for `HumanLanguageName::Hindi`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A4%A8%E0%A4%AE%E0%A4%B8%E0%A5%8D%E0%A4%A4%E0%A5%87#Hindi>
//! - <https://en.wiktionary.org/wiki/%E0%A4%A6%E0%A5%81%E0%A4%A8%E0%A4%BF%E0%A4%AF%E0%A4%BE#Hindi>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

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
