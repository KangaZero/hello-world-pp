//! Hello-world greetings for `HumanLanguageName::Assamese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%A6%AC%E0%A6%BF%E0%A6%B6%E0%A7%8D%E0%A6%AC#Assamese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Assamese => "Assamese" | "অসমীয়া",
    dialects: [StandardAssamese => "Standard Assamese"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Assamese,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Bengali, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardAssamese,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Bengali],
                text: "নমস্কাৰ, বিশ্ব!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "nomoskar, bisso!",
            },
        ],
    }],
};
