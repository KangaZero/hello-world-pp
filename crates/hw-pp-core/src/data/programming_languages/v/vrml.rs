//! Hello-world snippet for `ProgrammingLanguageName::Vrml`.
//!
//! Imported from leachim6/hello-world (`v/VRML.wrl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VRML.wrl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Markup],
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
