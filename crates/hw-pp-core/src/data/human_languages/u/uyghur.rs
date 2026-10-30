//! Hello-world greetings for `HumanLanguageName::Uyghur`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D8%AF%DB%87%D9%86%D9%8A%D8%A7#Uyghur>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::Turkic, HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Uyghur => "Uyghur" | "ئۇيغۇرچە",
    dialects: [StandardUyghur => "Standard Uyghur"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Uyghur,
    categories: &[Natural],
    families: &[Turkic],
    scripts: &[ScriptName::PersoArabic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardUyghur,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::PersoArabic],
                text: "سالام، دۇنيا!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Salam dunya!",
            },
        ],
    }],
};
