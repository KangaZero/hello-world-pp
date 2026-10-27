//! Hello-world greetings for `HumanLanguageName::LowGerman`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/moin#Low_German>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: LowGerman => "Low German" | "Plattdüütsch",
    dialects: [NorthernLowSaxon => "Northern Low Saxon"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::LowGerman,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::NorthernLowSaxon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Moin, Welt!",
        }],
    }],
};
