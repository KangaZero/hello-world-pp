//! Hello-world snippet for `ProgrammingLanguageName::Ked`.
//!
//! Imported from leachim6/hello-world (`k/Ked.ked` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Ked.ked>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ked => "Ked",
    versions: [Ked => "Ked"],
    extensions: [Ked => "ked"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ked,
        extensions: &[FileExtension::Ked],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ked,
            syntax: concat!(r"saysI '", output_for_hello_world!(), r"' like"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"saysI '", output_for_hello_world!(), r"' like"),
            )],
        }],
    };
