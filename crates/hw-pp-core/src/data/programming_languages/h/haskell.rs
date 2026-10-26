//! Hello-world snippet for `ProgrammingLanguageName::Haskell`.
//!
//! Sources:
//! - <https://www.haskell.org/onlinereport/haskell2010/>
//! - <https://www.haskell.org/tutorial/io.html>
//! - <https://www.wikidata.org/wiki/Q34010> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Haskell, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Haskell => "Haskell",
    versions: [Haskell2010 => "Haskell 2010"],
    extensions: [Hs => "hs", Lhs => "lhs"],
}

const HASKELL2010: &str = concat!(
    r#"main :: IO ()
main = putStrLn ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haskell,
        extensions: &[FileExtension::Hs, FileExtension::Lhs],
        categories: &[Compiled],
        families: &[Haskell],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haskell2010,
            syntax: HASKELL2010,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HASKELL2010,
                    [
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
                )
            },
        }],
    };
