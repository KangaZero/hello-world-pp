//! Hello-world snippet for `ProgrammingLanguageName::Mulisp`.
//!
//! Imported from the Hello World Collection (`files/MuLisp.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MuLisp.txt>
//! - <http://helloworldcollection.de/#MuLisp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mulisp => "MuLisp",
    versions: [Mulisp => "MuLisp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mulisp,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mulisp,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r" in MuLisp

(print 'Hello\,\ world\!)"
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"; ", output_for_hello_world!(), r" in MuLisp"),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" 'Hello\"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"\ world\!"),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
