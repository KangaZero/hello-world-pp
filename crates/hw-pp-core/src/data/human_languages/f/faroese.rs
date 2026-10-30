//! Hello-world greetings for `HumanLanguageName::Faroese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hall%C3%B3#Faroese>
//! - <https://en.wiktionary.org/wiki/heimur#Faroese>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Faroese => "Faroese" | "føroyskt",
    dialects: [StandardFaroese => "Standard Faroese"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Faroese,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardFaroese,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halló, heimur!",
        }],
    }],
};
