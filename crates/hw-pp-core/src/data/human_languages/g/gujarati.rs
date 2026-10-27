//! Hello-world greetings for `HumanLanguageName::Gujarati`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%AA%A8%E0%AA%AE%E0%AA%B8%E0%AB%8D%E0%AA%A4%E0%AB%87#Gujarati>
//! - <https://en.wiktionary.org/wiki/%E0%AA%B5%E0%AA%BF%E0%AA%B6%E0%AB%8D%E0%AA%B5#Gujarati>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{IndoAryan, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Gujarati => "Gujarati" | "ગુજરાતી",
    dialects: [StandardGujarati => "Standard Gujarati"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Gujarati,
    categories: &[Natural],
    families: &[IndoEuropean, IndoAryan],
    scripts: &[ScriptName::Gujarati, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardGujarati,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Gujarati],
                text: "નમસ્તે, વિશ્વ!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Namaste, viśva!",
            },
        ],
    }],
};
