//! Hello-world snippet for `ProgrammingLanguageName::Virgil`.
//!
//! Imported from leachim6/hello-world (`v/Virgil.v3` at `a152253`),
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
    language: Virgil => "Virgil",
    versions: [Virgil => "Virgil"],
    extensions: [V3 => "v3"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Virgil,
        extensions: &[FileExtension::V3],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Virgil,
            syntax: concat!(
                r#" def main() {
        System.puts(""#,
                output_for_hello_world!(),
                r#"\n");
 }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#" def main() {
        System.puts(""#,
                    output_for_hello_world!(),
                    r#"\n");
 }"#
                ),
            )],
        }],
    };
