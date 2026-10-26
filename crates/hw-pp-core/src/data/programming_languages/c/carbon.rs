//! Hello-world snippet for `ProgrammingLanguageName::Carbon`.
//!
//! Imported from leachim6/hello-world (`c/Carbon.carbon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Carbon.carbon>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Carbon => "Carbon",
    versions: [Carbon => "Carbon"],
    extensions: [Carbon => "carbon"],
}

const CARBON: &str = concat!(
    r#"package HelloWorld api;

fn Main() -> i32 {
  Print(""#,
    output_for_hello_world!(),
    r#"");
  return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Carbon,
        extensions: &[FileExtension::Carbon],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Carbon,
            syntax: CARBON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CARBON,
                    [
                        (TokenKind::Keyword, r"package"),
                        (TokenKind::Plain, r" HelloWorld api"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" -> "),
                        (TokenKind::Type, r"i32"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"Print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
