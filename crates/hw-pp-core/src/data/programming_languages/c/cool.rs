//! Hello-world snippet for `ProgrammingLanguageName::Cool`.
//!
//! Imported from leachim6/hello-world (`c/Cool.cl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cool.cl>
//! - <https://www.wikidata.org/wiki/Q2904782> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Educational},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cool => "Cool",
    versions: [Cool => "Cool"],
    extensions: [],
}

const COOL: &str = concat!(
    r#"class Main inherits IO {
   main(): Object {
	out_string(""#,
    output_for_hello_world!(),
    r#".\n")
   };
};"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cool,
        extensions: &[],
        categories: &[Compiled, Educational],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cool,
            syntax: COOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COOL,
                    [
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" Main inherits IO "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" Object "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Function, r"out_string"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#".\n""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Punctuation, r"};"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"};"),
                    ],
                )
            },
        }],
    };
