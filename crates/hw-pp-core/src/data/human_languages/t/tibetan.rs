//! Hello-world greetings for `HumanLanguageName::Tibetan`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E0%BD%96%E0%BD%80%E0%BE%B2%E0%BC%8B%E0%BD%A4%E0%BD%B2%E0%BD%A6%E0%BC%8B%E0%BD%96%E0%BD%91%E0%BD%BA%E0%BC%8B%E0%BD%A3%E0%BD%BA%E0%BD%82%E0%BD%A6#Tibetan>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Tibetan => "Tibetan" | "བོད་སྐད",
    dialects: [StandardTibetan => "Standard Tibetan"],
    scripts: [Tibetan => "Tibetan", Latin => "Latin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Tibetan,
    categories: &[Natural],
    scripts: &[ScriptName::Tibetan, ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardTibetan,
        greetings: &[
            Greeting {
                scripts: &[ScriptName::Tibetan],
                text: "བཀྲ་ཤིས་བདེ་ལེགས། འཛམ་གླིང་།",
            },
            Greeting {
                scripts: &[ScriptName::Latin],
                text: "bkra shis bde legs 'dzam gling!",
            },
        ],
    }],
};
