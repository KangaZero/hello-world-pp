//! Hello-world snippet for `ProgrammingLanguageName::Labyrinth`.
//!
//! Imported from leachim6/hello-world (`l/Labyrinth.labyrinth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Labyrinth => "Labyrinth",
    versions: [Labyrinth => "Labyrinth"],
    extensions: [Labyrinth => "labyrinth"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Labyrinth,
        extensions: &[FileExtension::Labyrinth],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Labyrinth,
            syntax: r"33_100_108:}_114_111:}_87_32_44{{:_101_72.......
@",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"33_100_108:}_114_111:}_87_32_44{{:_101_72.......
@",
            )],
        }],
    };
