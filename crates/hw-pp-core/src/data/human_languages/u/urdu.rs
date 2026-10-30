//! Hello-world greetings for `HumanLanguageName::Urdu`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D8%B3%D9%84%D8%A7%D9%85#Urdu>
//! - <https://en.wiktionary.org/wiki/%D8%AF%D9%86%DB%8C%D8%A7#Urdu>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Urdu => "Urdu" | "اردو",
    dialects: [StandardUrdu => "Standard Urdu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Urdu,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::PersoArabic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardUrdu,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::PersoArabic],
                text: "سلام، دنیا!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "salām duniyā!",
            },
        ],
    }],
};
