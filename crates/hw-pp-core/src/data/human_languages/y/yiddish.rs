//! Hello-world greetings for `HumanLanguageName::Yiddish`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D7%A9%D7%9C%D7%95%D7%9D%D6%BE%D7%A2%D7%9C%D7%99%D7%9B%D7%9D#Yiddish>
//! - <https://en.wiktionary.org/wiki/%D7%95%D7%95%D7%A2%D7%9C%D7%98#Yiddish>
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
    language: Yiddish => "Yiddish" | "ייִדיש",
    dialects: [StandardYiddish => "Standard Yiddish"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Yiddish,
    categories: &[Natural],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Hebrew, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardYiddish,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Hebrew],
                text: "שלום־עליכם, וועלט!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "sholem-aleykhem, velt!",
            },
        ],
    }],
};
