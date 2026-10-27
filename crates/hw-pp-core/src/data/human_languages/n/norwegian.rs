//! Hello-world greetings for `HumanLanguageName::Norwegian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hei#Norwegian_Bokm%C3%A5l>
//! - <https://en.wiktionary.org/wiki/verden#Norwegian_Bokm%C3%A5l>
//! - <https://en.wiktionary.org/wiki/verda#Norwegian_Bokm%C3%A5l>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Norwegian => "Norwegian" | "Norsk",
    dialects: [Bokmal => "Bokmål", Nynorsk => "Nynorsk"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Norwegian,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::Bokmal,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hei, verden!",
            }],
        },
        Dialect {
            label: DialectName::Nynorsk,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hei, verda!",
            }],
        },
    ],
};
