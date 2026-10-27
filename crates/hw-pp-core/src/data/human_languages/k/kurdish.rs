//! Hello-world greetings for `HumanLanguageName::Kurdish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D8%B3%DA%B5%D8%A7%D9%88#Central_Kurdish>
//! - <https://en.wiktionary.org/wiki/silav#Northern_Kurdish>
//! - <https://en.wiktionary.org/wiki/%D8%AC%DB%8C%D9%87%D8%A7%D9%86#Central_Kurdish>
//! - <https://en.wiktionary.org/wiki/c%C3%AEhan#Northern_Kurdish>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoEuropean, Iranian},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Kurdish => "Kurdish" | "Kurdî",
    dialects: [Kurmanji => "Kurmanji" | "Kurmancî", Sorani => "Sorani" | "سۆرانی"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Kurdish,
    categories: &[Natural],
    families: &[IndoEuropean, Iranian],
    scripts: &[ScriptName::Latin, ScriptName::Arabic],
    dialects: &[
        Dialect {
            label: DialectName::Kurmanji,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Silav, cîhan!",
            }],
        },
        Dialect {
            label: DialectName::Sorani,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "سڵاو، جیهان!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Silaw, cîhan!",
                },
            ],
        },
    ],
};
