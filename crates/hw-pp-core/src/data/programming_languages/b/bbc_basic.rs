//! Hello-world snippet for `ProgrammingLanguageName::BbcBasic`.
//!
//! Imported from leachim6/hello-world (`b/BBC BASIC.bbc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BBC%20BASIC.bbc>
//! - <https://en.wikipedia.org/wiki/BBC_BASIC> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Structured;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BbcBasic => "BBC BASIC",
    versions: [BbcBasic => "BBC BASIC"],
    extensions: [Bbc => "bbc"],
}

const BBC_BASIC: &str = concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BbcBasic,
        extensions: &[FileExtension::Bbc],
        categories: &[Historical],
        families: &[Basic],
        paradigms: &[Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BbcBasic,
            syntax: BBC_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BBC_BASIC,
                    [
                        (TokenKind::Keyword, r"PRINT"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
