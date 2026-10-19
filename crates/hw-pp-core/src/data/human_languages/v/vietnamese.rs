//! Hello-world greetings for `HumanLanguageName::Vietnamese`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Vietnamese => "Vietnamese" | "Tiếng Việt",
    dialects: [StandardVietnamese => "Standard Vietnamese"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Vietnamese,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardVietnamese,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Xin chào, thế giới!",
        }],
    }],
};
