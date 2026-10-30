//! Hello-world greetings for `HumanLanguageName::Belarusian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%BF%D1%80%D1%8B%D0%B2%D1%96%D1%82%D0%B0%D0%BD%D0%BD%D0%B5#Belarusian>
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B2%D0%B5%D1%82#Belarusian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Belarusian => "Belarusian" | "Беларуская",
    dialects: [StandardBelarusian => "Standard Belarusian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Belarusian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBelarusian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Прывітанне, свет!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Pryvitannie, sviet!",
            },
        ],
    }],
};
