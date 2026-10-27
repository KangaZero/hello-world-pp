//! Hello-world greetings for `HumanLanguageName::Irish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/dia#Irish>
//! - <https://en.wiktionary.org/wiki/A#Irish>
//! - <https://en.wiktionary.org/wiki/dhuit#Irish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Celtic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Irish => "Irish" | "Gaeilge",
    dialects: [StandardIrish => "Standard Irish" | "An Caighdeán Oifigiúil"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Irish,
    categories: &[Natural],
    families: &[IndoEuropean, Celtic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardIrish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Dia dhuit, a dhomhain!",
        }],
    }],
};
