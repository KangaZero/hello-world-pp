//! Hello-world snippet for `ProgrammingLanguageName::Kv`.
//!
//! Imported from leachim6/hello-world (`k/Kv.kv` at `a152253`),
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
    language: Kv => "Kv",
    versions: [Kv => "Kv"],
    extensions: [Kv => "kv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kv,
        extensions: &[FileExtension::Kv],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kv,
            syntax: concat!(
                r#"Label:
        text: ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Label:
        text: ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
