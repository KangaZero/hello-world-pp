//! Hello-world greetings for `HumanLanguageName::TokiPona`.
//!
//! Sources:
//! - <https://sona.pona.la/wiki/toki>
//! - <https://sona.pona.la/wiki/ma>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Constructed,
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: TokiPona => "Toki Pona",
    dialects: [StandardTokiPona => "Standard Toki Pona"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::TokiPona,
    categories: &[Constructed],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTokiPona,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "toki, ma!",
        }],
    }],
};
