//! Hello-world greetings for `HumanLanguageName::Czech`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Czech => "Czech" | "Čeština",
    dialects: [StandardCzech => "Standard Czech"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Czech,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardCzech,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Ahoj, světe!",
        }],
    }],
};
