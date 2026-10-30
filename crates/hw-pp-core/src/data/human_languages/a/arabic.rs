//! Hello-world greetings for `HumanLanguageName::Arabic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%D9%85%D8%B1%D8%AD%D8%A8%D8%A7#Arabic>
//! - <https://en.wiktionary.org/wiki/%D8%B9%D8%A7%D9%84%D9%85#Arabic>
//! - <https://en.wiktionary.org/wiki/%D8%A8%D8%A7%D9%84%D9%81%D8%B9%D9%84#Arabic>
//! - <https://en.wiktionary.org/wiki/%D8%A8#Egyptian_Arabic>
//! - <https://en.wiktionary.org/wiki/%D8%A7%D9%84#Egyptian_Arabic>
//! - <https://en.wiktionary.org/wiki/%D9%85%D8%B1%D8%AD%D8%A8%D8%A7#South_Levantine_Arabic>
//! - <https://en.wiktionary.org/wiki/%D8%B9%D8%A7%D9%84%D9%85#South_Levantine_Arabic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Afroasiatic, Semitic},
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Arabic => "Arabic" | "العربية",
    dialects: [ModernStandardArabic => "Modern Standard Arabic" | "العربية الفصحى", EgyptianArabic => "Egyptian Arabic" | "مصري", LevantineArabic => "Levantine Arabic" | "شامي"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Arabic,
    categories: &[Natural],
    families: &[Afroasiatic, Semitic],
    scripts: &[ScriptName::Arabic, ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::ModernStandardArabic,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "مرحبا بالعالم!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "marḥaban bi-l-ʕālam!",
                },
            ],
        },
        Dialect {
            label: DialectName::EgyptianArabic,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "أهلاً بالعالم!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "ahlan bil-ʕālam!",
                },
            ],
        },
        Dialect {
            label: DialectName::LevantineArabic,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::Arabic],
                    text: "مرحبا بالعالم!",
                },
                Greeting {
                    scripts: &[ScriptName::Latin],
                    text: "marḥaba bi-l-ʕālam!",
                },
            ],
        },
    ],
};
