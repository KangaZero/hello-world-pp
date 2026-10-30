//! Hello-world greetings for `HumanLanguageName::Nahuatl`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/niltze#Classical_Nahuatl>
//! - <https://en.wiktionary.org/wiki/cemanahuac#Classical_Nahuatl>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::UtoAztecan, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Nahuatl => "Nahuatl",
    dialects: [ModernNahuatl => "Modern Nahuatl"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Nahuatl,
    categories: &[Natural],
    families: &[UtoAztecan],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::ModernNahuatl,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Niltze, cemanahuac!",
        }],
    }],
};
