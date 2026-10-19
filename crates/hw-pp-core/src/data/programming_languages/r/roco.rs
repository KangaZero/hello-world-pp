//! Hello-world snippet for `ProgrammingLanguageName::Roco`.
//!
//! Imported from leachim6/hello-world (`r/Roco.roco` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Roco => "Roco",
    versions: [Roco => "Roco"],
    extensions: [Roco => "roco"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Roco,
        extensions: &[FileExtension::Roco],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Roco,
            syntax: r"cout 72
cout 101
cout 108
cout 108
cout 111
cout 44
cout 32
cout 87
cout 111
cout 114
cout 108
cout 100
cout 33
ac",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"cout 72
cout 101
cout 108
cout 108
cout 111
cout 44
cout 32
cout 87
cout 111
cout 114
cout 108
cout 100
cout 33
ac",
            )],
        }],
    };
