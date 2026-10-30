//! Hello-world snippet for `ProgrammingLanguageName::Fstar`.
//!
//! Imported from leachim6/hello-world (`f/FStar.fst` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FStar.fst>
//! - <https://www.wikidata.org/wiki/Q5423569> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::ProofAssistant,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fstar => "FStar",
    versions: [Fstar => "FStar"],
    extensions: [Fst => "fst"],
}

const FSTAR: &str = concat!(
    r#"module Hello

let main = FStar.IO.print_string ""#,
    output_for_hello_world!(),
    r#"\n""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fstar,
        extensions: &[FileExtension::Fst],
        categories: &[ProofAssistant],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fstar,
            syntax: FSTAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FSTAR,
                    [
                        (TokenKind::Keyword, r"module"),
                        (
                            TokenKind::Plain,
                            r" Hello

",
                        ),
                        (TokenKind::Keyword, r"let"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"main"),
                        (TokenKind::Plain, r" = FStar"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"IO"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"print_string"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                    ],
                )
            },
        }],
    };
