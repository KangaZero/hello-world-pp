//! Hello-world greetings for `HumanLanguageName::Kikuyu`.
//!
//! Imported from the Hello World Collection (`files/Human.htm` at `8e4c095`),
//! MIT licensed; see `human_languages/LICENSE-helloworldcollection`.
//! Changed from upstream: spells `thi` as `thĩ`, as in Kikuyu orthography.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/th%C4%A9#Kikuyu>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kikuyu => "Kikuyu" | "Gĩkũyũ",
    dialects: [StandardKikuyu => "Standard Kikuyu"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kikuyu,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardKikuyu,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Niatia thĩ!",
        }],
    }],
};
