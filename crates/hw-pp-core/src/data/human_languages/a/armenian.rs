//! Hello-world greetings for `HumanLanguageName::Armenian`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D5%A1%D5%B7%D5%AD%D5%A1%D6%80%D5%B0#Armenian>
//! - <https://en.wiktionary.org/wiki/%D5%A2%D5%A1%D6%80%D6%87#Armenian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageFamily::IndoEuropean, HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Armenian => "Armenian" | "Հայերեն",
    dialects: [EasternArmenian => "Eastern Armenian" | "Արևելահայերեն", WesternArmenian => "Western Armenian" | "Արեւմտահայերէն"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Armenian,
    categories: &[Natural],
    families: &[IndoEuropean],
    scripts: &[ScriptName::Armenian, ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::EasternArmenian,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Armenian],
                    text: "Բարեւ, աշխարհ!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Barew, ašxarh!",
                },
            ],
        },
        Dialect {
            label: DialectName::WesternArmenian,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Armenian],
                    text: "Բարեւ աշխարհ!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "Barew ašxarh!",
                },
            ],
        },
    ],
};
