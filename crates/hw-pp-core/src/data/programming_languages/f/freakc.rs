//! Hello-world snippet for `ProgrammingLanguageName::Freakc`.
//!
//! Imported from leachim6/hello-world (`f/FreakC.fclang` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FreakC.fclang>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Freakc => "FreakC",
    versions: [Freakc => "FreakC"],
    extensions: [Fclang => "fclang"],
}

const FREAKC: &str = concat!(r"print[] ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Freakc,
        extensions: &[FileExtension::Fclang],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Freakc,
            syntax: FREAKC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FREAKC,
                    [(
                        TokenKind::Plain,
                        concat!(r"print[] ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
