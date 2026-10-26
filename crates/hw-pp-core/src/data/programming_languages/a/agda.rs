//! Hello-world snippet for `ProgrammingLanguageName::Agda`.
//!
//! Imported from leachim6/hello-world (`a/Agda.agda` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Agda.agda>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::ProofAssistant,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Haskell, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Agda => "Agda",
    versions: [Agda => "Agda"],
    extensions: [Agda => "agda"],
}

const AGDA: &str = concat!(
    r#"module agda where
open import IO

main = run (putStrLn ""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Agda,
        extensions: &[FileExtension::Agda],
        categories: &[ProofAssistant],
        families: &[Haskell],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Agda,
            syntax: AGDA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AGDA,
                    [
                        (TokenKind::Keyword, r"module"),
                        (TokenKind::Plain, r" agda "),
                        (TokenKind::Keyword, r"where"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"open"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"IO"),
                        (
                            TokenKind::Plain,
                            r"

main = run ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"putStrLn"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
