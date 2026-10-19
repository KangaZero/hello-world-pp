//! Hello-world snippet for `ProgrammingLanguageName::AU02f8`.
//!
//! Imported from leachim6/hello-world (`a/A˸;` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/A%CB%B8%3B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AU02f8 => "A˸;",
    versions: [AU02f8 => "A˸;"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AU02f8,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AU02f8,
            syntax: concat!(r"j:", output_for_hello_world!(), r"\n;p:j;k"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"j:", output_for_hello_world!(), r"\n;p:j;k"),
            )],
        }],
    };
