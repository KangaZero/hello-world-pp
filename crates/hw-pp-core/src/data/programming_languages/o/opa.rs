//! Hello-world snippet for `ProgrammingLanguageName::Opa`.
//!
//! Imported from leachim6/hello-world (`o/Opa.opa` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Opa.opa>
//! - <https://www.wikidata.org/wiki/Q3353033> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Opa => "Opa",
    versions: [Opa => "Opa"],
    extensions: [Opa => "opa"],
}

const OPA: &str = concat!(r#"jlog(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Opa,
        extensions: &[FileExtension::Opa],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Opa,
            syntax: OPA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OPA,
                    [
                        (TokenKind::Function, r"jlog"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
