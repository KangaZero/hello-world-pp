//! Hello-world greetings for `HumanLanguageName::German`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: German => "German",
    dialects: [StandardGerman => "Standard German", SwissGerman => "Swiss German", Bavarian => "Bavarian"],
    scripts: [Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::German,
    categories: &[Natural],
    scripts: &[ScriptName::Latin],
    dialects: &[
        Dialect {
            label: DialectName::StandardGerman,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hallo, Welt!",
            }],
        },
        Dialect {
            label: DialectName::SwissGerman,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hallo, Wält!",
            }],
        },
        Dialect {
            label: DialectName::Bavarian,
            greetings: &[Greeting {
                scripts: &[ScriptName::Latin],
                text: "Servus, Wöd!",
            }],
        },
    ],
};
