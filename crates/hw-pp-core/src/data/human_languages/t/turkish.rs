//! Hello-world greetings for `HumanLanguageName::Turkish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/merhaba#Turkish>
//! - <https://en.wiktionary.org/wiki/d%C3%BCnya#Turkish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Turkic, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Turkish => "Turkish" | "Türkçe",
    dialects: [IstanbulTurkish => "Istanbul Turkish" | "İstanbul Türkçesi"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Turkish,
    categories: &[Natural],
    families: &[Turkic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::IstanbulTurkish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Merhaba, Dünya!",
        }],
    }],
};
