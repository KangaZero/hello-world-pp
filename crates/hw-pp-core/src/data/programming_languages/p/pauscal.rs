//! Hello-world snippet for `ProgrammingLanguageName::Pauscal`.
//!
//! Imported from leachim6/hello-world (`p/Pauscal.pdp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pauscal.pdp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pauscal => "Pauscal",
    versions: [Pauscal => "Pauscal"],
    extensions: [Pdp => "pdp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pauscal,
        extensions: &[FileExtension::Pdp],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pauscal,
            syntax: concat!(
                r#"Importar "Pauscal.prp"
Mensaje(""#,
                output_for_hello_world!(),
                r#"")
Fin"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Importar "Pauscal.prp"
Mensaje(""#,
                    output_for_hello_world!(),
                    r#"")
Fin"#
                ),
            )],
        }],
    };
