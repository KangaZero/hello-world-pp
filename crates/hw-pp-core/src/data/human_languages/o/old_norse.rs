//! Hello-world greetings for `HumanLanguageName::OldNorse`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/heill#Old_Norse>
//! - <https://en.wiktionary.org/wiki/heimr#Old_Norse>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Extinct,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: OldNorse => "Old Norse" | "norrœnt mál",
    dialects: [OldWestNorse => "Old West Norse"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::OldNorse,
    categories: &[Extinct],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::OldWestNorse,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Heill, heimr!",
        }],
    }],
};
