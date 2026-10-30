//! Hello-world greetings for `HumanLanguageName::Oromo`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/akkam#Oromo>
//! - <https://en.wiktionary.org/wiki/addunyaa#Oromo>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Cushitic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Oromo => "Oromo" | "Afaan Oromoo",
    dialects: [StandardOromo => "Standard Oromo"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Oromo,
    categories: &[Natural],
    families: &[Afroasiatic, Cushitic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardOromo,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Akkam, addunyaa!",
        }],
    }],
};
