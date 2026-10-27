//! Hello-world greetings for `HumanLanguageName::Kabyle`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/azul#Kabyle>
//! - <https://en.wiktionary.org/wiki/hello#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Berber},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kabyle => "Kabyle" | "Taqbaylit",
    dialects: [StandardKabyle => "Standard Kabyle"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kabyle,
    categories: &[Natural],
    families: &[Afroasiatic, Berber],
    scripts: &[ScriptName::Latin, ScriptName::Tifinagh],
    dialects: &[Dialect {
        label: DialectName::StandardKabyle,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Azul, amaḍal!",
            },
            Greeting {
                scripts: &[ScriptName::Tifinagh],
                text: "ⴰⵣⵓⵍ, ⴰⵎⴰⴹⴰⵍ!",
            },
        ],
    }],
};
