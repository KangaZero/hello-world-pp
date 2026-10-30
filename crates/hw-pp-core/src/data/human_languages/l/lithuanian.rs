//! Hello-world greetings for `HumanLanguageName::Lithuanian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/labas#Lithuanian>
//! - <https://en.wiktionary.org/wiki/pasauli#Lithuanian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Baltic, IndoEuropean},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Lithuanian => "Lithuanian" | "lietuvių kalba",
    dialects: [StandardLithuanian => "Standard Lithuanian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lithuanian,
    categories: &[Natural],
    families: &[IndoEuropean, Baltic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLithuanian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Labas, pasauli!",
        }],
    }],
};
