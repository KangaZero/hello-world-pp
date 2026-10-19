//! Hello-world greetings for `HumanLanguageName::French`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: French => "French" | "Français",
    dialects: [MetropolitanFrench => "Metropolitan French", QuebecoisFrench => "Québécois French" | "français québécois"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::French,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::MetropolitanFrench,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Bonjour le monde!",
            }],
        },
        Dialect {
            label: DialectName::QuebecoisFrench,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Bonjour le monde!",
            }],
        },
    ],
};
