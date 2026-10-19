//! Hello-world snippet for `ProgrammingLanguageName::One`.
//!
//! Imported from leachim6/hello-world (`o/One.one` at `a152253`),
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
    language: One => "One",
    versions: [One => "One"],
    extensions: [One => "one"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::One,
        extensions: &[FileExtension::One],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::One,
            syntax: concat!(
                r#"fn main {
	__ ""#,
                output_for_hello_world!(),
                r#""
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fn main {
	__ ""#,
                    output_for_hello_world!(),
                    r#""
}"#
                ),
            )],
        }],
    };
