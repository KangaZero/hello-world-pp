//! Hello-world greetings for `HumanLanguageName::LowerSorbian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/dobry#Lower_Sorbian>
//! - <https://en.wiktionary.org/wiki/%C5%BAe%C5%84#Lower_Sorbian>
//! - <https://en.wiktionary.org/wiki/sw%C4%9Bt#Lower_Sorbian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: LowerSorbian => "Lower Sorbian" | "dolnoserbšćina",
    dialects: [StandardLowerSorbian => "Standard Lower Sorbian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::LowerSorbian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLowerSorbian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Dobry źeń, swět!",
        }],
    }],
};
