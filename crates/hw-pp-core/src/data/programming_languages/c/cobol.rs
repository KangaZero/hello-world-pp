//! Hello-world snippet for `ProgrammingLanguageName::Cobol`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/COBOL>
//! - <https://gnucobol.sourceforge.io/>
//! - <https://www.wikidata.org/wiki/Q131140> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cobol => "COBOL",
    versions: [Cobol85 => "COBOL 85"],
    extensions: [Cob => "cob", Cbl => "cbl"],
}

const COBOL85: &str = concat!(
    r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
           DISPLAY ""#,
    output_for_hello_world!(),
    r#"".
           STOP RUN."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobol,
        extensions: &[FileExtension::Cob, FileExtension::Cbl],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobol85,
            syntax: COBOL85,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COBOL85,
                    [
                        (TokenKind::Plain, "       "),
                        (TokenKind::Keyword, "IDENTIFICATION DIVISION"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "\n       "),
                        (TokenKind::Keyword, "PROGRAM-ID"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, " HELLO"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "\n       "),
                        (TokenKind::Keyword, "PROCEDURE DIVISION"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "\n           "),
                        (TokenKind::Keyword, "DISPLAY"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Plain, "\n           "),
                        (TokenKind::Keyword, "STOP RUN"),
                        (TokenKind::Punctuation, "."),
                    ],
                )
            },
        }],
    };
