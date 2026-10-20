//! Hello-world snippet for `ProgrammingLanguageName::Picat`.
//!
//! Imported from leachim6/hello-world (`p/PiCat.pi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PiCat.pi>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Picat => "PiCat",
    versions: [Picat => "PiCat"],
    extensions: [Pi => "pi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Picat,
        extensions: &[FileExtension::Pi],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Picat,
            syntax: concat!(
                r#"main =>
  println(""#,
                output_for_hello_world!(),
                r#"")."#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"main"),
                (
                    TokenKind::Plain,
                    r" =>
  ",
                ),
                (TokenKind::Function, r"println"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")."),
            ],
        }],
    };
