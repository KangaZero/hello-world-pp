//! Hello-world snippet for `ProgrammingLanguageName::Gasoil`.
//!
//! Imported from leachim6/hello-world (`g/GASOIL.gasoil` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
