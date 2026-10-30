//! Hello-world greetings for `HumanLanguageName::Estonian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/tere#Estonian>
//! - <https://en.wiktionary.org/wiki/maailm#Estonian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Uralic, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Estonian => "Estonian" | "eesti keel",
    dialects: [StandardEstonian => "Standard Estonian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Estonian,
    categories: &[Natural],
    families: &[Uralic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardEstonian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Tere, maailm!",
        }],
    }],
};
