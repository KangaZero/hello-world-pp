//! Hello-world greetings for `HumanLanguageName::Greek`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Greek => "Greek",
    dialects: [ModernGreek => "Modern Greek"],
    scripts: [Greek => "Greek"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Greek,
    categories: &[Natural],
    scripts: &[ScriptName::Greek],
    dialects: &[Dialect {
        label: DialectName::ModernGreek,
        greetings: &[Greeting {
            scripts: &[ScriptName::Greek],
            text: "Γεια σου, Κόσμε!",
        }],
    }],
};
