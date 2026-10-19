//! Hello-world greetings for `HumanLanguageName::Sumerian`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Extinct,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Sumerian => "Sumerian",
    dialects: [Emegir => "Emegir"],
    scripts: [Cuneiform => "Cuneiform"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Sumerian,
    categories: &[Extinct],
    scripts: &[ScriptName::Cuneiform],
    dialects: &[Dialect {
        label: DialectName::Emegir,
        greetings: &[
            // silim an-ki: "well-being, heaven-earth"; a reconstruction, not attested
            Greeting {
                scripts: &[ScriptName::Cuneiform],
                text: "𒁲 𒀭𒆠",
            },
        ],
    }],
};
