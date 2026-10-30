//! Hello-world snippet for `ProgrammingLanguageName::Gleam`.
//!
//! Imported from leachim6/hello-world (`g/Gleam.gleam` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gleam.gleam>
//! - <https://www.wikidata.org/wiki/Q66370905> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gleam => "Gleam",
    versions: [Gleam => "Gleam"],
    extensions: [Gleam => "gleam"],
}

const GLEAM: &str = concat!(
    r#"import gleam/io

pub fn main() {
  io.println(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gleam,
        extensions: &[FileExtension::Gleam],
        categories: &[Compiled],
        families: &[JavaScript],
        paradigms: &[Functional, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gleam,
            syntax: GLEAM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GLEAM,
                    [
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" gleam/io

",
                        ),
                        (TokenKind::Keyword, r"pub"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  io",
                        ),
                        (TokenKind::Punctuation, r"."),
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
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
