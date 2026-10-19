//! Hello-world snippet for `ProgrammingLanguageName::Agda`.
//!
//! Imported from leachim6/hello-world (`a/Agda.agda` at `a152253`),
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
    language: Agda => "Agda",
    versions: [Agda => "Agda"],
    extensions: [Agda => "agda"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Agda,
        extensions: &[FileExtension::Agda],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Agda,
            syntax: concat!(
                r#"module agda where
open import IO

main = run (putStrLn ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module agda where
open import IO

main = run (putStrLn ""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
