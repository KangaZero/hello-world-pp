//! Hello-world greetings for `HumanLanguageName::Chinese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E6%B1%9D%E5%A5%BD#Chinese>
//! - <https://en.wiktionary.org/wiki/n%C7%90#Mandarin>
//! - <https://en.wiktionary.org/wiki/%E5%93%88%E5%9B%89#Cantonese>
//! - <https://en.wiktionary.org/wiki/%E4%BD%A0%E5%A5%BD#Cantonese>
//! - <https://en.wiktionary.org/wiki/h%C7%8Eo#Mandarin>
//! - <https://en.wiktionary.org/wiki/%E4%B8%96%E7%95%8C#Cantonese>
//! - <https://en.wiktionary.org/wiki/sh%C3%ACji%C3%A8#Mandarin>
//! - <https://en.wiktionary.org/wiki/hello#Translations>
//! - <https://en.wiktionary.org/wiki/world#Translations>

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Chinese => "Chinese" | "中文",
    dialects: [Mandarin => "Mandarin" | "普通话", Cantonese => "Cantonese" | "粵語", Hokkien => "Hokkien" | "閩南語"],
    scripts: [Han => "Han", SimplifiedChinese => "Simplified Chinese", TraditionalChinese => "Traditional Chinese", Latin => "Latin", Pinyin => "Pinyin"],
}

pub(in crate::data::human_languages) const LANGUAGE: HumanLanguage = HumanLanguage {
    name: HumanLanguageName::Chinese,
    categories: &[Natural],
    scripts: &[
        ScriptName::Han,
        ScriptName::SimplifiedChinese,
        ScriptName::TraditionalChinese,
        ScriptName::Latin,
        ScriptName::Pinyin,
    ],
    dialects: &[
        Dialect {
            label: DialectName::Mandarin,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::SimplifiedChinese],
                    text: "你好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::Pinyin],
                    text: "Nǐ hǎo, shìjiè!",
                },
            ],
        },
        Dialect {
            label: DialectName::Cantonese,
            greetings: &[Greeting {
                scripts: &[ScriptName::TraditionalChinese],
                text: "哈囉，世界！",
            }],
        },
        Dialect {
            label: DialectName::Hokkien,
            greetings: &[Greeting {
                scripts: &[ScriptName::TraditionalChinese],
                text: "汝好，世界！",
            }],
        },
    ],
};
