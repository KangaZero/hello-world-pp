//! Hello-world snippet for `ProgrammingLanguageName::U16b1U16a2U16beU16c5U16e6`.
//!
//! Imported from leachim6/hello-world (`#/ᚱᚢᚾᛅᛦ` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E1%9A%B1%E1%9A%A2%E1%9A%BE%E1%9B%85%E1%9B%A6>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: U16b1U16a2U16beU16c5U16e6 => "ᚱᚢᚾᛅᛦ",
    versions: [U16b1U16a2U16beU16c5U16e6 => "ᚱᚢᚾᛅᛦ"],
    extensions: [],
}

const U16B1_U16A2_U16BE_U16C5_U16E6: &str = r"ᛋᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭ
ᛁ͗̿ᛁ̊ᛁ̳͗̿ᛁ̹̊ᚹᛁ̿ᛁ̊ᛁ͗ᛁ̊ᛁ̹̊ᛁ̊̿ᛧ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U16b1U16a2U16beU16c5U16e6,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U16b1U16a2U16beU16c5U16e6,
            syntax: U16B1_U16A2_U16BE_U16C5_U16E6,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U16B1_U16A2_U16BE_U16C5_U16E6,
                    [(
                        TokenKind::Plain,
                        r"ᛋᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭᚭ
ᛁ͗̿ᛁ̊ᛁ̳͗̿ᛁ̹̊ᚹᛁ̿ᛁ̊ᛁ͗ᛁ̊ᛁ̹̊ᛁ̊̿ᛧ",
                    )],
                )
            },
        }],
    };
