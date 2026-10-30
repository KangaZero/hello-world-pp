//! Hello-world snippet for `ProgrammingLanguageName::CamlLight`.
//!
//! Imported from the Hello World Collection (`files/CAML-Light` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CAML-Light>
//! - <http://helloworldcollection.de/#CAML-Light>
//! - <https://en.wikipedia.org/wiki/Caml> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: CamlLight => "CAML-Light",
    versions: [CamlLight => "CAML-Light"],
    extensions: [],
}

const CAML_LIGHT: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in CAML Light *)

let hello =
   print_string ""#,
    output_for_hello_world!(),
    r#"";
;;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CamlLight,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CamlLight,
            syntax: CAML_LIGHT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CAML_LIGHT,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"(* ", output_for_hello_world!(), r" in CAML Light *)"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"let"),
                        (
                            TokenKind::Plain,
                            r" hello =
   ",
                        ),
                        (TokenKind::Function, r"print_string"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r";;"),
                    ],
                )
            },
        }],
    };
