//! Hello-world snippet for `ProgrammingLanguageName::Gasoil`.
//!
//! Imported from leachim6/hello-world (`g/GASOIL.gasoil` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GASOIL.gasoil>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gasoil => "GASOIL",
    versions: [Gasoil => "GASOIL"],
    extensions: [Gasoil => "gasoil"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gasoil,
        extensions: &[FileExtension::Gasoil],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gasoil,
            syntax: concat!(
                r#"main
(""#,
                output_for_hello_world!(),
                r#""; WRITE)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main
(""#,
                    output_for_hello_world!(),
                    r#""; WRITE)"#
                ),
            )],
        }],
    };
