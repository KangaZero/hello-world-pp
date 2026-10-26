//! Hello-world snippet for `ProgrammingLanguageName::PqnProc`.
//!
//! Imported from the Hello World Collection (`files/PQN-PROC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PQN-PROC.txt>
//! - <http://helloworldcollection.de/#PQN-PROC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PqnProc => "PQN-PROC",
    versions: [PqnProc => "PQN-PROC"],
    extensions: [],
}

const PQN_PROC: &str = concat!(
    r"PQN
C ",
    output_for_hello_world!(),
    r" in PQN/PROC
O ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PqnProc,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PqnProc,
            syntax: PQN_PROC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PQN_PROC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"PQN
C ",
                            output_for_hello_world!(),
                            r" in PQN/PROC
O ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
