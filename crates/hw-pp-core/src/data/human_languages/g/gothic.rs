//! Hello-world greetings for `HumanLanguageName::Gothic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%F0%90%8C%B7%F0%90%8C%B0%F0%90%8C%B9%F0%90%8C%BB%F0%90%8D%83#Gothic>
//! - <https://en.wiktionary.org/wiki/hails#Gothic>
//! - <https://en.wiktionary.org/wiki/%F0%90%8C%BC%F0%90%8C%B9%F0%90%8C%B3%F0%90%8C%BE%F0%90%8C%BF%F0%90%8C%BD%F0%90%8C%B2%F0%90%8C%B0%F0%90%8D%82%F0%90%8C%B3#Gothic>
//! - <https://en.wiktionary.org/wiki/midjungard#Gothic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Extinct,
    HumanLanguageFamily::{Germanic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Gothic => "Gothic" | "𐌲𐌿𐍄𐌹𐍃𐌺",
    dialects: [BiblicalGothic => "Biblical Gothic"],
    scripts: [Gothic => "Gothic", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Gothic,
    categories: &[Extinct],
    families: &[IndoEuropean, Germanic],
    scripts: &[ScriptName::Gothic, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::BiblicalGothic,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Gothic],
                text: "𐌷𐌰𐌹𐌻𐍃, 𐌼𐌹𐌳𐌾𐌿𐌽𐌲𐌰𐍂𐌳!",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "Hails, midjungard!",
            },
        ],
    }],
};
