//! Hello-world snippet for `ProgrammingLanguageName::Matlab10`.
//!
//! Imported from leachim6/hello-world (`m/MATLAB 1.0.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Matlab10 => "MATLAB 1.0",
    versions: [Matlab10 => "MATLAB 1.0"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab10,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab10,
            syntax: concat!(r"fprintf('", output_for_hello_world!(), r"\n')"),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"fprintf"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"\n'"),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
