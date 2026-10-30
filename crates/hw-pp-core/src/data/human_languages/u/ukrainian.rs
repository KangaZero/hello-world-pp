//! Hello-world greetings for `HumanLanguageName::Ukrainian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%BF%D1%80%D0%B8%D0%B2%D1%96%D1%82#Ukrainian>
//! - <https://en.wiktionary.org/wiki/%D1%81%D0%B2%D1%96%D1%82%D0%B5#Ukrainian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://czo.gov.ua/en/translit#:~:text=To%20address%20this%2C%20an%20official%20transliteration%20table%20was,the%20transliteration%20of%20the%20Ukrainian%20alphabet%20in%20Latin%22%29.>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Ukrainian => "Ukrainian" | "Українська",
    dialects: [StandardUkrainian => "Standard Ukrainian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Ukrainian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardUkrainian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Привіт, світе!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Pryvit, svite!",
            },
        ],
    }],
};
