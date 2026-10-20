//! Hello-world snippet for `ProgrammingLanguageName::U6613U8bedU8a00`.
//!
//! Imported from leachim6/hello-world (`#/易语言.e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E6%98%93%E8%AF%AD%E8%A8%80.e>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: U6613U8bedU8a00 => "易语言",
    versions: [U6613U8bedU8a00 => "易语言"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U6613U8bedU8a00,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U6613U8bedU8a00,
            syntax: concat!(r"调试输出(“", output_for_hello_world!(), r"”）"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"调试输出(“", output_for_hello_world!(), r"”）"),
            )],
        }],
    };
