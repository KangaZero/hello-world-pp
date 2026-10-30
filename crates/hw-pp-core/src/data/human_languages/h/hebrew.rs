//! Hello-world greetings for `HumanLanguageName::Hebrew`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D7%A9%D7%9C%D7%95%D7%9D#Hebrew>
//! - <https://en.wiktionary.org/wiki/%D7%A2%D7%95%D7%9C%D7%9D#Hebrew>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Hebrew => "Hebrew" | "עברית",
    dialects: [ModernHebrew => "Modern Hebrew"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hebrew,
    categories: &[Natural],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Hebrew, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ModernHebrew,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Hebrew],
                text: "שלום, עולם!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "shalóm, 'olám!",
            },
        ],
    }],
};
