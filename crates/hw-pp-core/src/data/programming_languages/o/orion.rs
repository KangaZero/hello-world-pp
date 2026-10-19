//! Hello-world snippet for `ProgrammingLanguageName::Orion`.
//!
//! Imported from leachim6/hello-world (`o/Orion.orn` at `a152253`),
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
    language: Orion => "Orion",
    versions: [Orion => "Orion"],
    extensions: [Orn => "orn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Orion,
        extensions: &[FileExtension::Orn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Orion,
            syntax: concat!(
                r#"(def 'impure main
     (λ () (putStrLn ""#,
                output_for_hello_world!(),
                r#"")))"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"(def 'impure main
     (λ () (putStrLn ""#,
                    output_for_hello_world!(),
                    r#"")))"#
                ),
            )],
        }],
    };
