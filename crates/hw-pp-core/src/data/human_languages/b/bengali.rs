//! Hello-world greetings for `HumanLanguageName::Bengali`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A6%B9%E0%A7%8D%E0%A6%AF%E0%A6%BE%E0%A6%B2%E0%A7%8B#Bengali>
//! - <https://en.wiktionary.org/wiki/%E0%A6%AC%E0%A6%BF%E0%A6%B6%E0%A7%8D%E0%A6%AC#Bengali>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Bengali => "Bengali" | "বাংলা",
    dialects: [StandardBengali => "Standard Bengali"],
    scripts: [Bengali => "Bengali"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bengali,
    categories: &[Natural],
    scripts: &[ScriptName::Bengali],
    dialects: &[Dialect {
        label: DialectName::StandardBengali,
        greetings: &[Greeting {
            scripts: &[ScriptName::Bengali],
            text: "হ্যালো, বিশ্ব!",
        }],
    }],
};
