//! Hello-world snippets for `ProgrammingLanguageName::Abap`.
//!
//! Imported from leachim6/hello-world (`a/ABAP.abap` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ABAP4.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ABAP.abap>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ABAP4.txt>
//! - <http://helloworldcollection.de/#ABAP4>
//! - <https://www.wikidata.org/wiki/Q380523> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Abap => "ABAP",
    versions: [Abap => "ABAP", Abap4 => "ABAP4"],
    extensions: [Abap => "abap"],
}

const ABAP: &str = concat!(
    r"REPORT ZHELLO_WORLD.

START-OF-SELECTION.
    WRITE: '",
    output_for_hello_world!(),
    r"'.

"
);
const ABAP4: &str = concat!(
    r"REPORT ZHB00001.
*",
    output_for_hello_world!(),
    r" in ABAP/4 *
WRITE: '",
    output_for_hello_world!(),
    r"'."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Abap,
        extensions: &[FileExtension::Abap],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Structured, ObjectOriented],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Abap,
                syntax: ABAP,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ABAP,
                        [
                            (TokenKind::Keyword, "REPORT"),
                            (TokenKind::Plain, " ZHELLO_WORLD"),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Plain, "\n\n"),
                            (TokenKind::Keyword, "START-OF-SELECTION"),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Plain, "\n    "),
                            (TokenKind::Keyword, "WRITE"),
                            (TokenKind::Punctuation, ":"),
                            (TokenKind::Plain, " "),
                            (
                                TokenKind::String,
                                concat!("'", output_for_hello_world!(), "'"),
                            ),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Plain, "\n\n"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Abap4,
                syntax: ABAP4,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ABAP4,
                        [
                            (TokenKind::Keyword, "REPORT"),
                            (TokenKind::Plain, " ZHB00001"),
                            (TokenKind::Punctuation, "."),
                            (TokenKind::Plain, "\n"),
                            // A `*` in the first column makes the whole line a comment.
                            (
                                TokenKind::Comment,
                                concat!("*", output_for_hello_world!(), " in ABAP/4 *"),
                            ),
                            (TokenKind::Plain, "\n"),
                            (TokenKind::Keyword, "WRITE"),
                            (TokenKind::Punctuation, ":"),
                            (TokenKind::Plain, " "),
                            (
                                TokenKind::String,
                                concat!("'", output_for_hello_world!(), "'"),
                            ),
                            (TokenKind::Punctuation, "."),
                        ],
                    )
                },
            },
        ],
    };
