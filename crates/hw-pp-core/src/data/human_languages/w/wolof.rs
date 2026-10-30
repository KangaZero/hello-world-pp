//! Hello-world greetings for `HumanLanguageName::Wolof`.
//!
//! Sources:
//! - <https://wo.wikipedia.org/wiki/Esperanto#Salaamaalekum>
//! - <https://wo.wikipedia.org/wiki/%C3%80dduna>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::NigerCongo, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Wolof => "Wolof",
    dialects: [StandardWolof => "Standard Wolof"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Wolof,
    categories: &[Natural],
    families: &[NigerCongo],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardWolof,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salaamaalekum, àdduna!",
        }],
    }],
};
