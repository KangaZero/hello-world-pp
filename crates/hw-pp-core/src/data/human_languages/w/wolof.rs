//! Hello-world greetings for `HumanLanguageName::Wolof`.
//!
//! Sources:
//! - <https://wo.wikipedia.org/wiki/Esperanto#Salaamaalekum>
//! - <https://wo.wikipedia.org/wiki/%C3%80dduna>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Wolof => "Wolof",
    dialects: [StandardWolof => "Standard Wolof"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Wolof,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardWolof,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salaamaalekum, àdduna!",
        }],
    }],
};
