//! Hello-world snippets for `ProgrammingLanguageName::Haskell`.
//!
//! Sources:
//! - <https://www.haskell.org/onlinereport/haskell2010/>
//! - <https://www.haskell.org/tutorial/io.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Haskell => "Haskell",
    versions: [Haskell2010 => "Haskell 2010"],
    extensions: [Hs => "hs", Lhs => "lhs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haskell,
        extensions: &[FileExtension::Hs, FileExtension::Lhs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haskell2010,
            syntax: concat!(
                r#"main :: IO ()
main = putStrLn ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Function, "main"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "::"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "IO"),
                (TokenKind::Plain, " ()\nmain "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, " putStrLn "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };
