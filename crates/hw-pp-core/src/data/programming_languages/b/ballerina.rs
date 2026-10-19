//! Hello-world snippet for `ProgrammingLanguageName::Ballerina`.
//!
//! Imported from leachim6/hello-world (`b/Ballerina.bal` at `a152253`),
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
    language: Ballerina => "Ballerina",
    versions: [Ballerina => "Ballerina"],
    extensions: [Bal => "bal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ballerina,
        extensions: &[FileExtension::Bal],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ballerina,
            syntax: concat!(
                r#"import ballerina/io;

public function main() {
    io:println(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import ballerina/io;

public function main() {
    io:println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
