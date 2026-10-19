//! Hello-world greetings for `HumanLanguageName::Armenian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Armenian => "Armenian",
    dialects: [EasternArmenian => "Eastern Armenian", WesternArmenian => "Western Armenian"],
    scripts: [Armenian => "Armenian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Armenian,
    categories: &[Natural],
    scripts: &[ScriptName::Armenian],
    dialects: &[
        Dialect {
            label: DialectName::EasternArmenian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Armenian],
                text: "Բարեւ, աշխարհ!",
            }],
        },
        Dialect {
            label: DialectName::WesternArmenian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Armenian],
                text: "Բարեւ աշխարհ!",
            }],
        },
    ],
};
