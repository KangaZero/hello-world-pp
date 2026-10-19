//! Hello-world snippet for `ProgrammingLanguageName::Cesil`.
//!
//! Imported from leachim6/hello-world (`c/CESIL.cesil` at `a152253`),
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
    language: Cesil => "CESIL",
    versions: [Cesil => "CESIL"],
    extensions: [Cesil => "cesil"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cesil,
        extensions: &[FileExtension::Cesil],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cesil,
            syntax: concat!(
                r#"        PRINT   ""#,
                output_for_hello_world!(),
                r#""
        HALT
%
*"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"        PRINT   ""#,
                    output_for_hello_world!(),
                    r#""
        HALT
%
*"#
                ),
            )],
        }],
    };
