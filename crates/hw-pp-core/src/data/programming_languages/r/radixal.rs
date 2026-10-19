//! Hello-world snippet for `ProgrammingLanguageName::Radixal`.
//!
//! Imported from leachim6/hello-world (`r/Radixal.radixal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Radixal => "Radixal",
    versions: [Radixal => "Radixal"],
    extensions: [Radixal => "radixal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Radixal,
        extensions: &[FileExtension::Radixal],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Radixal,
            syntax: r"1 3 2 44 15 27 15 4 2 34 15 34 15 4 4 121 15 4 2 8 15
4 4 45 15 121 15 3 4 56 15 1102 15 46 15 3 2 15 15 3 4 5 15",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"1 3 2 44 15 27 15 4 2 34 15 34 15 4 4 121 15 4 2 8 15
4 4 45 15 121 15 3 4 56 15 1102 15 46 15 3 2 15 15 3 4 5 15",
            )],
        }],
    };
