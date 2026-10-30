//! Hello-world greetings for `HumanLanguageName::Occitan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mond#Occitan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Italic, Romance},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Occitan => "Occitan",
    dialects: [Languedocien => "Languedocien" | "lengadocian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Occitan,
    categories: &[Natural],
    families: &[IndoEuropean, Italic, Romance],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::Languedocien,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Bonjorn, mond!",
        }],
    }],
};
