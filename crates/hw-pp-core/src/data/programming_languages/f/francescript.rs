//! Hello-world snippet for `ProgrammingLanguageName::Francescript`.
//!
//! Imported from leachim6/hello-world (PR #1482 `f/FranceScript.fr`),
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
    language: Francescript => "FranceScript",
    versions: [Francescript => "FranceScript"],
    extensions: [Fr => "fr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Francescript,
        extensions: &[FileExtension::Fr],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Francescript,
            syntax: concat!(
                r#"console->log(""#,
                output_for_hello_world!(),
                r#" !") fin"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"console->log(""#,
                    output_for_hello_world!(),
                    r#" !") fin"#
                ),
            )],
        }],
    };
