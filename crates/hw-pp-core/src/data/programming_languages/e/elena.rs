//! Hello-world snippet for `ProgrammingLanguageName::Elena`.
//!
//! Imported from leachim6/hello-world (`e/Elena.elena` at `a152253`),
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
    language: Elena => "Elena",
    versions: [Elena => "Elena"],
    extensions: [Elena => "elena"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elena,
        extensions: &[FileExtension::Elena],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elena,
            syntax: concat!(
                r#"public program
[
    console writeLine(""#,
                output_for_hello_world!(),
                r#"").
]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"public program
[
    console writeLine(""#,
                    output_for_hello_world!(),
                    r#"").
]"#
                ),
            )],
        }],
    };
