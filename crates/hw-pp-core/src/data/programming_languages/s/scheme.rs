//! Hello-world snippet for `ProgrammingLanguageName::Scheme`.
//!
//! Sources:
//! - <https://small.r7rs.org/attachment/r7rs.pdf>
//! - <https://r7rs.org/>
//! - <https://en.wikipedia.org/wiki/Scheme_(programming_language)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Scheme => "Scheme",
    versions: [R7rs => "R7RS"],
    extensions: [Scm => "scm", Ss => "ss"],
}

const R7RS: &str = concat!(
    r#"(display ""#,
    output_for_hello_world!(),
    r#"")
(newline)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scheme,
        extensions: &[FileExtension::Scm, FileExtension::Ss],
        categories: &[Compiled, Historical],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::R7rs,
            syntax: R7RS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    R7RS,
                    [
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Plain, "display "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ")"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Plain, "newline"),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };
