//! Hello-world snippet for `ProgrammingLanguageName::Xlisp`.
//!
//! Imported from leachim6/hello-world (`x/XLisp.xlisp` at `a152253`),
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
    language: Xlisp => "XLisp",
    versions: [Xlisp => "XLisp"],
    extensions: [Xlisp => "xlisp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xlisp,
        extensions: &[FileExtension::Xlisp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xlisp,
            syntax: concat!(
                r#"(DISPLAY ""#,
                output_for_hello_world!(),
                r#"")
(NEWLINE)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(DISPLAY ""#,
                    output_for_hello_world!(),
                    r#"")
(NEWLINE)"#
                ),
            )],
        }],
    };
