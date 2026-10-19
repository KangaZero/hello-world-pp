//! Hello-world greetings for `HumanLanguageName::Norwegian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Norwegian => "Norwegian" | "Norsk",
    dialects: [Bokmal => "Bokmål", Nynorsk => "Nynorsk"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Norwegian,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::Bokmal,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hei, verden!",
            }],
        },
        Dialect {
            label: DialectName::Nynorsk,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hei, verda!",
            }],
        },
    ],
};
