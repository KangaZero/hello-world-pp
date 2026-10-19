//! Hello-world greetings for `HumanLanguageName::Lojban`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Constructed,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Lojban => "Lojban" | "la .lojban.",
    dialects: [StandardLojban => "Standard Lojban"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lojban,
    categories: &[Constructed],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLojban,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "coi munje",
        }],
    }],
};
