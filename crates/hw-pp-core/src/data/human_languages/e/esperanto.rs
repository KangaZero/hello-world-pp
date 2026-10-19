//! Hello-world greetings for `HumanLanguageName::Esperanto`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Constructed,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Esperanto => "Esperanto",
    dialects: [StandardEsperanto => "Standard Esperanto"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Esperanto,
    categories: &[Constructed],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardEsperanto,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Saluton, mondo!",
        }],
    }],
};
