//! Hello-world greetings for `HumanLanguageName::ScottishGaelic`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/hal%C3%B2#Scottish_Gaelic>
//! - <https://en.wiktionary.org/wiki/A#Scottish_Gaelic>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Human.htm>
//! - <http://helloworldcollection.de/#Human>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage,
    HumanLanguageCategory::Natural,
    HumanLanguageFamily::{Celtic, IndoEuropean},
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: ScottishGaelic => "Scottish Gaelic" | "Gàidhlig",
    dialects: [StandardScottishGaelic => "Standard Scottish Gaelic"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::ScottishGaelic,
    categories: &[Natural],
    families: &[IndoEuropean, Celtic],
    scripts: &[ScriptName::Latin],
    dialects: &[Dialect {
        label: DialectName::StandardScottishGaelic,
        greetings: &[Greeting {
            scripts: &[ScriptName::Latin],
            text: "Halò, a shaoghail!",
        }],
    }],
};
