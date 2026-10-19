//! Hello-world greetings for `HumanLanguageName::NorthernSotho`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Northern_Sotho_language>
//! - <https://nso.wikipedia.org/wiki/Lefase>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: NorthernSotho => "Northern Sotho" | "Sesotho sa Leboa",
    dialects: [StandardNorthernSotho => "Standard Northern Sotho"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::NorthernSotho,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardNorthernSotho,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Thobela, lefase!",
        }],
    }],
};
