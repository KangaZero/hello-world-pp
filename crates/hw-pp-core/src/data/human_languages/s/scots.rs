//! Hello-world greetings for `HumanLanguageName::Scots`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/warld#Scots>
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
    language: Scots => "Scots",
    dialects: [StandardScots => "Standard Scots"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Scots,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardScots,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hullo, warld!",
        }],
    }],
};
