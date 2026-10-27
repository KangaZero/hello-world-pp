//! Hello-world snippet for `ProgrammingLanguageName::Hamler`.
//!
//! Imported from leachim6/hello-world (`h/Hamler.hm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hamler.hm>
//! - <https://github.com/hamler-lang/hamler> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hamler => "Hamler",
    versions: [Hamler => "Hamler"],
    extensions: [Hm => "hm"],
}

const HAMLER: &str = concat!(
    r#"module Main where
import Prelude
main = print ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hamler,
        extensions: &[FileExtension::Hm],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Functional, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hamler,
            syntax: HAMLER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HAMLER,
                    [
                        (TokenKind::Keyword, r"module"),
                        (TokenKind::Plain, r" Main "),
                        (TokenKind::Keyword, r"where"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" Prelude
main = ",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
