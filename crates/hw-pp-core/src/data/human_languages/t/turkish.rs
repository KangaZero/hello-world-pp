//! Hello-world greetings for `HumanLanguageName::Turkish`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Turkish => "Turkish" | "Türkçe",
    dialects: [IstanbulTurkish => "Istanbul Turkish" | "İstanbul Türkçesi"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Turkish,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::IstanbulTurkish,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Merhaba, Dünya!",
        }],
    }],
};
