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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gasoil => "GASOIL",
    versions: [Gasoil => "GASOIL"],
    extensions: [Gasoil => "gasoil"],
}

const GASOIL: &str = concat!(
    r#"main
(""#,
    output_for_hello_world!(),
    r#""; WRITE)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gasoil,
        extensions: &[FileExtension::Gasoil],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gasoil,
            syntax: GASOIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GASOIL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"main
(""#,
                            output_for_hello_world!(),
                            r#""; WRITE)"#
                        ),
                    )],
                )
            },
        }],
    };
