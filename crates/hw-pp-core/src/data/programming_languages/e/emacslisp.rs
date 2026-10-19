//! Hello-world snippet for `ProgrammingLanguageName::Emacslisp`.
//!
//! Imported from leachim6/hello-world (`e/EmacsLisp.el` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Emacslisp => "EmacsLisp",
    versions: [Emacslisp => "EmacsLisp"],
    extensions: [El => "el"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emacslisp,
        extensions: &[FileExtension::El],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emacslisp,
            syntax: concat!(r#"(message ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"message "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
