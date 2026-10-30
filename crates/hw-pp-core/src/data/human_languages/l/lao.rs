//! Hello-world greetings for `HumanLanguageName::Lao`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%BA%AA%E0%BA%B0%E0%BA%9A%E0%BA%B2%E0%BA%8D%E0%BA%94%E0%BA%B5#Lao>
//! - <https://en.wiktionary.org/wiki/%E0%BB%82%E0%BA%A5%E0%BA%81#Lao>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::KraDai, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Lao => "Lao" | "ພາສາລາວ",
    dialects: [StandardLao => "Standard Lao"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lao,
    categories: &[Natural],
    families: &[KraDai],
    scripts: &[ScriptName::Lao, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLao,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Lao],
                text: "ສະບາຍດີ, ໂລກ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "sabaidi, lok!",
            },
        ],
    }],
};
