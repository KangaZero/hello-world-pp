//! Hello-world snippets for `ProgrammingLanguageName::Python`.
//!
//! Sources:
//! - <https://docs.python.org/3/tutorial/introduction.html>
//! - <https://docs.python.org/2/reference/simple_stmts.html#the-print-statement>
//! - <https://docs.python.org/3/whatsnew/3.0.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Python => "Python",
    versions: [Python2 => "Python 2", Python3 => "Python 3"],
    extensions: [Py => "py"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Python,
        extensions: &[FileExtension::Py],
        categories: &[Scripting],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python2,
                syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "print"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Python3,
                syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                tokenized_syntax: &[
                    (TokenKind::Function, "print"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ")"),
                ],
            },
        ],
    };
