//! Hello-world greetings for `HumanLanguageName::Chinese`.
//!
//! Sources:
//! - <https://en.wiktionary.org/wiki/%E4%BD%A0%E5%A5%BD#Chinese>
//! - <https://en.wiktionary.org/wiki/%E5%93%88%E5%9B%89#Chinese>
//! - <https://en.wiktionary.org/wiki/%E6%B1%9D%E5%A5%BD#Chinese>
//! - <https://en.wiktionary.org/wiki/%E5%84%82%E5%A5%BD#Chinese>
//! - <https://en.wiktionary.org/wiki/%E4%BE%AC%E5%A5%BD#Chinese>
//! - <https://en.wiktionary.org/wiki/%E4%B8%96%E7%95%8C#Chinese>
//! - <https://en.wiktionary.org/wiki/n%C7%90#Mandarin>
//! - <https://en.wiktionary.org/wiki/h%C7%8Eo#Mandarin>
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
    dialects: [
        Mandarin => "Mandarin" | "普通话",
        Cantonese => "Cantonese" | "粵語",
        Hokkien => "Hokkien" | "閩南語",
        Shanghainese => "Shanghainese" | "上海話",
        Teochew => "Teochew" | "潮州話",
        Fuzhounese => "Fuzhounese" | "福州話",
    ],
    scripts: [
        Han => "Han",
        SimplifiedChinese => "Simplified Chinese",
        TraditionalChinese => "Traditional Chinese",
        Latin => "Latin",
        Pinyin => "Pinyin",
        Jyutping => "Jyutping",
        PehOeJi => "Pe̍h-ōe-jī",
        Wugniu => "Wugniu",
        Pengim => "Peng'im",
        FoochowRomanized => "Foochow Romanized",
    ],
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
        ScriptName::Jyutping,
        ScriptName::PehOeJi,
        ScriptName::Wugniu,
        ScriptName::Pengim,
        ScriptName::FoochowRomanized,
    ],
    dialects: &[
        Dialect {
            label: DialectName::Mandarin,
            greetings: &[
                Greeting {
                    scripts: &[
                        ScriptName::SimplifiedChinese,
                        ScriptName::TraditionalChinese,
                    ],
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
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::TraditionalChinese],
                    text: "哈囉，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::Jyutping],
                    text: "Haa1 lo2, sai3 gaai3!",
                },
            ],
        },
        Dialect {
            label: DialectName::Hokkien,
            greetings: &[
                Greeting {
                    scripts: &[
                        ScriptName::TraditionalChinese,
                        ScriptName::SimplifiedChinese,
                    ],
                    text: "汝好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::PehOeJi],
                    text: "Lí hó, sè-kài!",
                },
            ],
        },
        Dialect {
            label: DialectName::Shanghainese,
            greetings: &[
                Greeting {
                    scripts: &[ScriptName::SimplifiedChinese],
                    text: "侬好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::TraditionalChinese],
                    text: "儂好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::Wugniu],
                    text: "⁶non ⁵hau, ¹sy₅-ka!",
                },
            ],
        },
        Dialect {
            label: DialectName::Teochew,
            greetings: &[
                Greeting {
                    scripts: &[
                        ScriptName::SimplifiedChinese,
                        ScriptName::TraditionalChinese,
                    ],
                    text: "汝好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::Pengim],
                    text: "Le2 ho2, si3 gai3!",
                },
            ],
        },
        Dialect {
            label: DialectName::Fuzhounese,
            greetings: &[
                Greeting {
                    scripts: &[
                        ScriptName::SimplifiedChinese,
                        ScriptName::TraditionalChinese,
                    ],
                    text: "汝好，世界！",
                },
                Greeting {
                    scripts: &[ScriptName::Latin, ScriptName::FoochowRomanized],
                    text: "Nṳ̄ hō̤, sié-gái!",
                },
            ],
        },
    ],
};
