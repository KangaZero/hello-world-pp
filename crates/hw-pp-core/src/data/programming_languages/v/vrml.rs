//! Hello-world snippet for `ProgrammingLanguageName::Vrml`.
//!
//! Imported from leachim6/hello-world (`v/VRML.wrl` at `a152253`),
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
    language: Vrml => "VRML",
    versions: [Vrml => "VRML"],
    extensions: [Wrl => "wrl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vrml,
        extensions: &[FileExtension::Wrl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vrml,
            syntax: concat!(
                r#"#VRML V2.0 utf8
Shape {
    geometry Text {
        string ""#,
                output_for_hello_world!(),
                r#""
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#VRML V2.0 utf8
Shape {
    geometry Text {
        string ""#,
                    output_for_hello_world!(),
                    r#""
    }
}"#
                ),
            )],
        }],
    };
