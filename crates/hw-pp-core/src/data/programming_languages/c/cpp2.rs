//! Hello-world snippet for `ProgrammingLanguageName::Cpp2`.
//!
//! Imported from leachim6/hello-world (PR #1572 `c/c++2.cpp2`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1572>
//! - <https://en.wikipedia.org/wiki/C%2B%2B> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Generic, Imperative, ObjectOriented, Procedural, Reflective};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cpp2 => "Cpp2",
    versions: [Cpp2 => "Cpp2"],
    extensions: [Cpp2 => "cpp2"],
}

const CPP2: &str = concat!(
    r#"main: () = std::println(""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cpp2,
        extensions: &[FileExtension::Cpp2],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, Procedural, ObjectOriented, Functional, Reflective, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cpp2,
            syntax: CPP2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CPP2,
                    [
                        (TokenKind::Plain, r"main"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" = std"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
