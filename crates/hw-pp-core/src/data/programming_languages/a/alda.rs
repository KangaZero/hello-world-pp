//! Hello-world snippet for `ProgrammingLanguageName::Alda`.
//!
//! Imported from leachim6/hello-world (`a/Alda.alda` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Alda => "Alda",
    versions: [Alda => "Alda"],
    extensions: [Alda => "alda"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alda,
        extensions: &[FileExtension::Alda],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alda,
            syntax: r"piano: c8 d e f g f e d c2.",
            tokenized_syntax: &[(TokenKind::Plain, r"piano: c8 d e f g f e d c2.")],
        }],
    };
