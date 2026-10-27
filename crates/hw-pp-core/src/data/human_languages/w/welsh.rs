//! Hello-world greetings for `HumanLanguageName::Welsh`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/helo#Welsh>
//! - <https://en.wiktionary.org/wiki/fyd#Welsh>
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
    language: Welsh => "Welsh" | "Cymraeg",
    dialects: [StandardWelsh => "Standard Welsh"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Welsh,
    categories: &[Natural],
    families: &[IndoEuropean, Celtic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardWelsh,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Helo, fyd!",
        }],
    }],
};
