//! Hello-world snippet for `ProgrammingLanguageName::Hexish`.
//!
//! Imported from leachim6/hello-world (`h/Hexish.hexish` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hexish => "Hexish",
    versions: [Hexish => "Hexish"],
    extensions: [Hexish => "hexish"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexish,
        extensions: &[FileExtension::Hexish],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexish,
            syntax: r"0902160E48656C6C6F20776F726C6421001001161508",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"0902160E48656C6C6F20776F726C6421001001161508",
            )],
        }],
    };
