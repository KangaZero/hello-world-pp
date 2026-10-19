//! Hello-world greetings for `HumanLanguageName::Esperanto`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/saluton#Esperanto>
//! - <https://en.wiktionary.org/wiki/mondo#Esperanto>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::{Auxiliary, Constructed},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Esperanto => "Esperanto",
    dialects: [StandardEsperanto => "Standard Esperanto"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Esperanto,
    categories: &[Auxiliary, Constructed],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardEsperanto,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Saluton, mondo!",
        }],
    }],
};
