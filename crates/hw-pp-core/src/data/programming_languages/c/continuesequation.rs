//! Hello-world snippet for `ProgrammingLanguageName::Continuesequation`.
//!
//! Imported from leachim6/hello-world (`c/ContinuesEquation.ce` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/ContinuesEquation.ce>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Continuesequation => "ContinuesEquation",
    versions: [Continuesequation => "ContinuesEquation"],
    extensions: [Ce => "ce"],
}

const CONTINUESEQUATION: &str = r"0 0 o'H'e'l'l'o' 'W'o'r'l'd";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Continuesequation,
        extensions: &[FileExtension::Ce],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Continuesequation,
            syntax: CONTINUESEQUATION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CONTINUESEQUATION,
                    [(TokenKind::Plain, r"0 0 o'H'e'l'l'o' 'W'o'r'l'd")],
                )
            },
        }],
    };
