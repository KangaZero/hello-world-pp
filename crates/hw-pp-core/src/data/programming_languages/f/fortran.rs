//! Hello-world snippet for `ProgrammingLanguageName::Fortran`.
//!
//! Sources:
//! - <https://fortran-lang.org/learn/quickstart/hello_world/>
//! - <https://en.wikipedia.org/wiki/Fortran>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Fortran,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fortran => "Fortran",
    versions: [Fortran90 => "Fortran 90"],
    extensions: [F => "f", F90 => "f90", F95 => "f95"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fortran,
        extensions: &[FileExtension::F, FileExtension::F90, FileExtension::F95],
        categories: &[Compiled, Historical],
        families: &[Fortran],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fortran90,
            syntax: concat!(
                r#"program hello
    print *, ""#,
                output_for_hello_world!(),
                r#""
end program hello"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " hello\n    "),
                (TokenKind::Keyword, "print"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "*,"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "end"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "program"),
                (TokenKind::Plain, " hello"),
            ],
        }],
    };
