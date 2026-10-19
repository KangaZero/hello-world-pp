//! Hello-world snippet for `ProgrammingLanguageName::U79c0U4e38U30deU30afU30ed`.
//!
//! Imported from leachim6/hello-world (`#/秀丸マクロ.mac` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E7%A7%80%E4%B8%B8%E3%83%9E%E3%82%AF%E3%83%AD.mac>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: U79c0U4e38U30deU30afU30ed => "秀丸マクロ",
    versions: [U79c0U4e38U30deU30afU30ed => "秀丸マクロ"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U79c0U4e38U30deU30afU30ed,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U79c0U4e38U30deU30afU30ed,
            syntax: concat!(r#"message ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"message ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
