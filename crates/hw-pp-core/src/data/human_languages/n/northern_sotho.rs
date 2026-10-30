//! Hello-world greetings for `HumanLanguageName::NorthernSotho`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Northern_Sotho_language>
//! - <https://nso.wikipedia.org/wiki/Lefase>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: NorthernSotho => "Northern Sotho" | "Sesotho sa Leboa",
    dialects: [StandardNorthernSotho => "Standard Northern Sotho"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::NorthernSotho,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardNorthernSotho,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Thobela, lefase!",
        }],
    }],
};
