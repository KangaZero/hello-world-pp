//! Hello-world snippet for `ProgrammingLanguageName::Continuesequation`.
//!
//! Imported from leachim6/hello-world (`c/ContinuesEquation.ce` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Continuesequation => "ContinuesEquation",
    versions: [Continuesequation => "ContinuesEquation"],
    extensions: [Ce => "ce"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Continuesequation,
        extensions: &[FileExtension::Ce],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Continuesequation,
            syntax: r"0 0 o'H'e'l'l'o' 'W'o'r'l'd",
            tokenized_syntax: &[(TokenKind::Plain, r"0 0 o'H'e'l'l'o' 'W'o'r'l'd")],
        }],
    };
