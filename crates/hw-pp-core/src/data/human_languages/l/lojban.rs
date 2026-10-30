//! Hello-world greetings for `HumanLanguageName::Lojban`.
//!
//! Sources:
//! - <https://jbovlaste.lojban.org/dict/coi>
//! - <https://jbovlaste.lojban.org/dict/munje>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Constructed,
    HumanLanguageName, ScriptName,
};

use hw_pp_macros::language_names;

language_names! {
    language: Lojban => "Lojban" | "la .lojban.",
    dialects: [StandardLojban => "Standard Lojban"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Lojban,
    categories: &[Constructed],
    families: &[],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardLojban,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "coi munje",
        }],
    }],
};
