//! Hello-world greetings for `HumanLanguageName::Arabic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D9%85%D8%B1%D8%AD%D8%A8%D8%A7#Arabic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Arabic => "Arabic" | "العربية",
    dialects: [ModernStandardArabic => "Modern Standard Arabic" | "العربية الفصحى", EgyptianArabic => "Egyptian Arabic" | "مصري", LevantineArabic => "Levantine Arabic" | "شامي"],
    scripts: [Arabic => "Arabic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Arabic,
    categories: &[Natural],
    scripts: &[ScriptName::Arabic],
    dialects: &[
        Dialect {
            label: DialectName::ModernStandardArabic,
            greetings: &[Greeting {
                scripts: &[ScriptName::Arabic],
                text: "مرحبا بالعالم!",
            }],
        },
        Dialect {
            label: DialectName::EgyptianArabic,
            greetings: &[Greeting {
                scripts: &[ScriptName::Arabic],
                text: "أهلاً بالعالم!",
            }],
        },
        Dialect {
            label: DialectName::LevantineArabic,
            greetings: &[Greeting {
                scripts: &[ScriptName::Arabic],
                text: "مرحبا بالعالم!",
            }],
        },
    ],
};
