//! Hello-world greetings for `HumanLanguageName::Cebuano`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/kumusta#Cebuano>
//! - <https://en.wiktionary.org/wiki/kalibotan#Cebuano>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Austronesian, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Cebuano => "Cebuano" | "Sinugboanon",
    dialects: [StandardCebuano => "Standard Cebuano"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Cebuano,
    categories: &[Natural],
    families: &[Austronesian],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardCebuano,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Kumusta, kalibotan!",
        }],
    }],
};
