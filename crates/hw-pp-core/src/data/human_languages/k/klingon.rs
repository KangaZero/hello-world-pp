//! Hello-world greetings for `HumanLanguageName::Klingon`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Constructed,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Klingon => "Klingon",
    dialects: [StandardKlingon => "Standard Klingon"],
    scripts: [Latin => "Latin", Piqad => "pIqaD"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Klingon,
    categories: &[Constructed],
    scripts: &[ScriptName::Latin, ScriptName::Piqad],
    dialects: &[Dialect {
        label: DialectName::StandardKlingon,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "nuqneH, qo'!",
        }],
    }],
};
