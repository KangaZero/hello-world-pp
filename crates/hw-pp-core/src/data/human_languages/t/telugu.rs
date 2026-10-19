//! Hello-world greetings for `HumanLanguageName::Telugu`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Telugu => "Telugu" | "తెలుగు",
    dialects: [StandardTelugu => "Standard Telugu"],
    scripts: [Telugu => "Telugu"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Telugu,
    categories: &[Natural],
    scripts: &[ScriptName::Telugu],
    dialects: &[Dialect {
        label: DialectName::StandardTelugu,
        greetings: &[Greeting {
            scripts: &[ScriptName::Telugu],
            text: "హలో, ప్రపంచం!",
        }],
    }],
};
