//! Hello-world greetings for `HumanLanguageName::German`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hallo#German>
//! - <https://en.wiktionary.org/wiki/servus#Bavarian>
//! - <https://en.wiktionary.org/wiki/Welt#German>
//! - <https://en.wiktionary.org/wiki/W%C3%B6d#Bavarian>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: German => "German" | "Deutsch",
    dialects: [StandardGerman => "Standard German" | "Hochdeutsch", SwissGerman => "Swiss German" | "Schwiizerdütsch", Bavarian => "Bavarian" | "Boarisch"],
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
