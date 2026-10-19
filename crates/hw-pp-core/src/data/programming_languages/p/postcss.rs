//! Hello-world snippet for `ProgrammingLanguageName::Postcss`.
//!
//! Imported from leachim6/hello-world (`p/PostCSS.pcss` at `a152253`),
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
    language: Postcss => "PostCSS",
    versions: [Postcss => "PostCSS"],
    extensions: [Pcss => "pcss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Postcss,
        extensions: &[FileExtension::Pcss],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Postcss,
            syntax: concat!(
                r#"body::before {
    content: ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"body::before {
    content: ""#,
                    output_for_hello_world!(),
                    r#"";
}"#
                ),
            )],
        }],
    };
