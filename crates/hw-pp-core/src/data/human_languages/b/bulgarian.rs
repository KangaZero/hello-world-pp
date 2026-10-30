//! Hello-world greetings for `HumanLanguageName::Bulgarian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%B7%D0%B4%D1%80%D0%B0%D0%B2%D0%B5%D0%B9#Bulgarian>
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B2%D1%8F%D1%82#Bulgarian>
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
    language: Bulgarian => "Bulgarian" | "Български",
    dialects: [StandardBulgarian => "Standard Bulgarian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bulgarian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardBulgarian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Здравей, свят!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Zdravey, svyat!",
            },
        ],
    }],
};
