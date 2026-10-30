//! Hello-world greetings for `HumanLanguageName::Xhosa`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/molo#Xhosa>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Xhosa => "Xhosa" | "isiXhosa",
    dialects: [StandardXhosa => "Standard Xhosa"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Xhosa,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardXhosa,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Molo, Hlabathi!",
        }],
    }],
};
