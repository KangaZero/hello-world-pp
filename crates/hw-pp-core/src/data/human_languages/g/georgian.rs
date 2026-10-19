//! Hello-world greetings for `HumanLanguageName::Georgian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E1%83%92%E1%83%90%E1%83%9B%E1%83%90%E1%83%A0%E1%83%AF%E1%83%9D%E1%83%91%E1%83%90#Georgian>
//! - <https://en.wiktionary.org/wiki/%E1%83%9B%E1%83%A1%E1%83%9D%E1%83%A4%E1%83%9A%E1%83%98%E1%83%9D#Georgian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Georgian => "Georgian" | "ქართული",
    dialects: [StandardGeorgian => "Standard Georgian"],
    scripts: [Georgian => "Georgian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Georgian,
    categories: &[Natural],
    scripts: &[ScriptName::Georgian],
    dialects: &[Dialect {
        label: DialectName::StandardGeorgian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Georgian],
            text: "გამარჯობა, მსოფლიო!",
        }],
    }],
};
