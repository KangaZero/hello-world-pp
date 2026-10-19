//! Hello-world greetings for `HumanLanguageName::Romanian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Romanian => "Romanian",
    dialects: [StandardRomanian => "Standard Romanian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Romanian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardRomanian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Salut, lume!",
        }],
    }],
};
