//! Hello-world snippet for `ProgrammingLanguageName::Ubiquity`.
//!
//! Imported from leachim6/hello-world (`u/Ubiquity.ubiquity` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Ubiquity.ubiquity>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ubiquity => "Ubiquity",
    versions: [Ubiquity => "Ubiquity"],
    extensions: [Ubiquity => "ubiquity"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ubiquity,
        extensions: &[FileExtension::Ubiquity],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ubiquity,
            syntax: concat!(
                r#"function cmd_hello_world() {
  displayMessage(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"function cmd_hello_world() {
  displayMessage(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
