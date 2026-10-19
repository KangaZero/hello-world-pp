//! Hello-world greetings for `HumanLanguageName::Persian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Persian => "Persian" | "فارسی",
    dialects: [IranianPersian => "Iranian Persian"],
    scripts: [PersoArabic => "Perso-Arabic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Persian,
    categories: &[Natural],
    scripts: &[ScriptName::PersoArabic],
    dialects: &[Dialect {
        label: DialectName::IranianPersian,
        greetings: &[Greeting {
            scripts: &[ScriptName::PersoArabic],
            text: "سلام دنیا!",
        }],
    }],
};
