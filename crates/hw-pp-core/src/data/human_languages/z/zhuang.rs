//! Hello-world greetings for `HumanLanguageName::Zhuang`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/mwngz#Zhuang>
//! - <https://en.wiktionary.org/wiki/ndei#Zhuang>
//! - <https://en.wiktionary.org/wiki/lajmbwn#Zhuang>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Zhuang => "Zhuang" | "Vahcuengh",
    dialects: [StandardZhuang => "Standard Zhuang"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Zhuang,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardZhuang,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Mwngz ndei, lajmbwn!",
        }],
    }],
};
