//! Hello-world greetings for `HumanLanguageName::Macedonian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D0%B7%D0%B4%D1%80%D0%B0%D0%B2%D0%BE#Macedonian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Slavic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Macedonian => "Macedonian" | "македонски",
    dialects: [StandardMacedonian => "Standard Macedonian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Macedonian,
    categories: &[Natural],
    families: &[IndoEuropean, Slavic],
    scripts: &[ScriptName::Cyrillic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardMacedonian,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Cyrillic],
                text: "Здраво, свету!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Zdravo, svetu!",
            },
        ],
    }],
};
