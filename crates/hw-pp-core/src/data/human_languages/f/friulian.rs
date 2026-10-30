//! Hello-world greetings for `HumanLanguageName::Friulian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mond#Friulian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Friulian => "Friulian" | "furlan",
    dialects: [StandardFriulian => "Standard Friulian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Friulian,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFriulian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mandi, mond!",
        }],
    }],
};
