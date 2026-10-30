//! Hello-world snippet for `ProgrammingLanguageName::Cangjie`.
//!
//! Imported from leachim6/hello-world (PR #1823 `c/Cangjie.cj`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1823>
//! - <https://en.wikipedia.org/wiki/Cangjie_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, Functional, Generic, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cangjie => "Cangjie",
    versions: [Cangjie => "Cangjie"],
    extensions: [Cj => "cj"],
}

const CANGJIE: &str = concat!(
    r#"main(): Int64 {
    println(""#,
    output_for_hello_world!(),
    r#"")
    return 0
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cangjie,
        extensions: &[FileExtension::Cj],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, Declarative, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cangjie,
            syntax: CANGJIE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CANGJIE,
                    [
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" Int64 "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
