//! Hello-world greetings for `HumanLanguageName::Georgian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Georgian => "Georgian" | "ქართული",
    dialects: [StandardGeorgian => "Standard Georgian"],
    scripts: [Georgian => "Georgian"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Georgian,
    categories: &[Natural],
    scripts: &[ScriptName::Georgian],
    dialects: &[Dialect {
        label: DialectName::StandardGeorgian,
        greetings: &[Greeting {
            scripts: &[ScriptName::Georgian],
            text: "გამარჯობა, მსოფლიო!",
        }],
    }],
};
