//! Hello-world greetings for `HumanLanguageName::Bemba`.
//!
//! Imported from the Hello World Collection (`files/Human.htm` at `8e4c095`),
//! MIT licensed; see `human_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/Bemba_language>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Bantu, NigerCongo},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Bemba => "Bemba" | "Ichibemba",
    dialects: [StandardBemba => "Standard Bemba"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bemba,
    categories: &[Natural],
    families: &[NigerCongo, Bantu],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBemba,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Shani Mwechalo!",
        }],
    }],
};
