//! Hello-world snippet for `ProgrammingLanguageName::Pauscal`.
//!
//! Imported from leachim6/hello-world (`p/Pauscal.pdp` at `a152253`),
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
    language: Pauscal => "Pauscal",
    versions: [Pauscal => "Pauscal"],
    extensions: [Pdp => "pdp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pauscal,
        extensions: &[FileExtension::Pdp],
        categories: &[],
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
