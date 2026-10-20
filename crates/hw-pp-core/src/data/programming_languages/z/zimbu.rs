//! Hello-world snippet for `ProgrammingLanguageName::Zimbu`.
//!
//! Imported from leachim6/hello-world (`z/Zimbu.zu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zimbu.zu>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zimbu => "Zimbu",
    versions: [Zimbu => "Zimbu"],
    extensions: [Zu => "zu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zimbu,
        extensions: &[FileExtension::Zu],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zimbu,
            syntax: concat!(
                r#"FUNC Main() int
  IO.write(""#,
                output_for_hello_world!(),
                r#"\n")
  RETURN 0
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"FUNC "),
                (TokenKind::Function, r"Main"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r" int
  IO",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"write"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
  RETURN ",
                ),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
