//! Hello-world greetings for `HumanLanguageName::Arabic`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Arabic => "Arabic",
    dialects: [ModernStandardArabic => "Modern Standard Arabic", EgyptianArabic => "Egyptian Arabic", LevantineArabic => "Levantine Arabic"],
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
