//! Hello-world snippet for `ProgrammingLanguageName::Realbasic`.
//!
//! Imported from leachim6/hello-world (`r/RealBasic.realbasic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RealBasic.realbasic>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Realbasic => "RealBasic",
    versions: [Realbasic => "RealBasic"],
    extensions: [Realbasic => "realbasic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Realbasic,
        extensions: &[FileExtension::Realbasic],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Realbasic,
            syntax: concat!(
                r#"Function Run(args() as String) As Integer
  Print ""#,
                output_for_hello_world!(),
                r#""
  Quit
End Function"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"Function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Run"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"args"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"as"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"String"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"As"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"Integer"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"Print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
  Quit
",
                ),
                (TokenKind::Keyword, r"End"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Function"),
            ],
        }],
    };
