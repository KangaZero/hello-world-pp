//! Hello-world snippet for `ProgrammingLanguageName::Matlab10`.
//!
//! Imported from leachim6/hello-world (`m/MATLAB 1.0.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MATLAB%201.0.m>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scientific},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Matlab10 => "MATLAB 1.0",
    versions: [Matlab10 => "MATLAB 1.0"],
    extensions: [],
}

const MATLAB10: &str = concat!(r"fprintf('", output_for_hello_world!(), r"\n')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab10,
        extensions: &[],
        categories: &[Scientific, Historical],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab10,
            syntax: MATLAB10,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATLAB10,
                    [
                        (TokenKind::Keyword, r"fprintf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"\n'"),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
