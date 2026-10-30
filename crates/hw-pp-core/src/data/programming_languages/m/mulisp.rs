//! Hello-world snippet for `ProgrammingLanguageName::Mulisp`.
//!
//! Imported from the Hello World Collection (`files/MuLisp.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MuLisp.txt>
//! - <http://helloworldcollection.de/#MuLisp>
//! - <https://en.wikipedia.org/wiki/Lisp_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Procedural, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mulisp => "MuLisp",
    versions: [Mulisp => "MuLisp"],
    extensions: [],
}

const MULISP: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in MuLisp

(print 'Hello\,\ world\!)"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mulisp,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, Functional, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mulisp,
            syntax: MULISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MULISP,
                    [
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
                )
            },
        }],
    };
