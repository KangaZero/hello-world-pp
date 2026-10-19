//! Hello-world greetings for `HumanLanguageName::Bengali`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Bengali => "Bengali" | "বাংলা",
    dialects: [StandardBengali => "Standard Bengali"],
    scripts: [Bengali => "Bengali"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Bengali,
    categories: &[Natural],
    scripts: &[ScriptName::Bengali],
    dialects: &[Dialect {
        label: DialectName::StandardBengali,
        greetings: &[Greeting {
            scripts: &[ScriptName::Bengali],
            text: "হ্যালো, বিশ্ব!",
        }],
    }],
};
