//! Hello-world greetings for `HumanLanguageName::Kazakh`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D1%81%D3%99%D0%BB%D0%B5%D0%BC#Kazakh>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Turkic, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Kazakh => "Kazakh" | "қазақ тілі",
    dialects: [StandardKazakh => "Standard Kazakh"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kazakh,
    categories: &[Natural],
    families: &[Turkic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKazakh,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Сәлем, әлем!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Sälem, älem!",
            },
        ],
    }],
};
