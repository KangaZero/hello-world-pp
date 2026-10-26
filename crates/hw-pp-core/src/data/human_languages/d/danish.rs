//! Hello-world greetings for `HumanLanguageName::Danish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hej#Danish>
//! - <https://en.wiktionary.org/wiki/verden#Danish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Danish => "Danish" | "Dansk",
    dialects: [StandardDanish => "Standard Danish"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Danish,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardDanish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Hej, verden!",
        }],
    }],
};
