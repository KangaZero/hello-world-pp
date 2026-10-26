//! Hello-world snippet for `ProgrammingLanguageName::Matlab`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/MATLAB>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Matlab => "MATLAB",
    versions: [Matlab => "MATLAB"],
    extensions: [M => "m"],
}

const MATLAB: &str = concat!(r"disp('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &[FileExtension::M],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab,
            syntax: MATLAB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATLAB,
                    [
                        (TokenKind::Keyword, "disp"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("'", output_for_hello_world!(), "'"),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };
