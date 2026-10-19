//! Hello-world greetings for `HumanLanguageName::Hebrew`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Hebrew => "Hebrew",
    dialects: [ModernHebrew => "Modern Hebrew"],
    scripts: [Hebrew => "Hebrew"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Hebrew,
    categories: &[Natural],
    scripts: &[ScriptName::Hebrew],
    dialects: &[Dialect {
        label: DialectName::ModernHebrew,
        greetings: &[Greeting {
            scripts: &[ScriptName::Hebrew],
            text: "שלום, עולם!",
        }],
    }],
};
