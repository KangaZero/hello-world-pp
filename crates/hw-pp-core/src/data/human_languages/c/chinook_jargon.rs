//! Hello-world greetings for `HumanLanguageName::ChinookJargon`.
//!
//! Imported from the Hello World Collection (`files/Human.htm` at `8e4c095`),
//! MIT licensed; see `human_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/MV_Klahowya>
//! - <https://en.wiktionary.org/wiki/klaska#Chinook_Jargon>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Pidgin,
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: ChinookJargon => "Chinook Jargon" | "Chinuk Wawa",
    dialects: [GrandRondeChinookJargon => "Grand Ronde Chinook Jargon"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::ChinookJargon,
    categories: &[Pidgin],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::GrandRondeChinookJargon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Klahowya Hayas Klaska!",
        }],
    }],
};
