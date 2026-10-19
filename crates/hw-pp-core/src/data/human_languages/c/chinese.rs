//! Hello-world greetings for `HumanLanguageName::Chinese`.

use crate::data::human_languages::{
    Dialect, DialectName, Greeting, HumanLanguage, HumanLanguageCategory::Natural,
    HumanLanguageName, ScriptName,
};
use crate::data::macros::language_names;

language_names! {
    language: Chinese => "Chinese",
    dialects: [Mandarin => "Mandarin", Cantonese => "Cantonese", Hokkien => "Hokkien"],
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
