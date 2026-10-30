//! Hello-world snippet for `ProgrammingLanguageName::Dafny`.
//!
//! Imported from the Hello World Collection (`files/Dafny.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Dafny.htm>
//! - <http://helloworldcollection.de/#Dafny>
//! - <https://www.wikidata.org/wiki/Q48989398> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dafny => "Dafny",
    versions: [Dafny => "Dafny"],
    extensions: [],
}

const DAFNY: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Dafny

method Main() {
  print ""#,
    output_for_hello_world!(),
    r#"\n";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dafny,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dafny,
            syntax: DAFNY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DAFNY,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"// ", output_for_hello_world!(), r" in Dafny"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

method ",
                        ),
                        (TokenKind::Function, r"Main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r";"),
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
