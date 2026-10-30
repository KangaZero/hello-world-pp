//! Hello-world greetings for `HumanLanguageName::Odia`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%AC%A8%E0%AC%AE%E0%AC%B8%E0%AD%8D%E0%AC%95%E0%AC%BE%E0%AC%B0#Odia>
//! - <https://en.wiktionary.org/wiki/%E0%AC%A6%E0%AD%81%E0%AC%A8%E0%AC%BF%E0%AC%86#Odia>
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
    language: Odia => "Odia" | "ଓଡ଼ିଆ",
    dialects: [StandardOdia => "Standard Odia"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Odia,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Odia, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardOdia,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Odia],
                text: "ନମସ୍କାର, ଦୁନିଆ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "namaskāra, duniā!",
            },
        ],
    }],
};
