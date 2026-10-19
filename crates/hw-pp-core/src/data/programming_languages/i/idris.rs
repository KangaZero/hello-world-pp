//! Hello-world snippet for `ProgrammingLanguageName::Idris`.
//!
//! Imported from leachim6/hello-world (`i/Idris.idr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Idris.idr>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, ProofAssistant},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Idris => "Idris",
    versions: [Idris => "Idris"],
    extensions: [Idr => "idr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idris,
        extensions: &[FileExtension::Idr],
        categories: &[Compiled, ProofAssistant],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idris,
            syntax: concat!(
                r#"module Main

main : IO ()
main = putStrLn ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module Main

main : IO ()
main = putStrLn ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
