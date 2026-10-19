//! Hello-world snippet for `ProgrammingLanguageName::DdDd`.
//!
//! Imported from leachim6/hello-world (`d/DD DD.dd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: DdDd => "DD DD",
    versions: [DdDd => "DD DD"],
    extensions: [Dd => "dd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DdDd,
        extensions: &[FileExtension::Dd],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DdDd,
            syntax: r"<!Hello\ World\0A;",
            tokenized_syntax: &[(TokenKind::Plain, r"<!Hello\ World\0A;")],
        }],
    };
