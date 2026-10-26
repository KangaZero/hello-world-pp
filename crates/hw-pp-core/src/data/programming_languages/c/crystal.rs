//! Hello-world snippet for `ProgrammingLanguageName::Crystal`.
//!
//! Sources:
//! - <https://crystal-lang.org/reference/latest/getting_started/>
//! - <https://crystal-lang.org/reference/latest/tutorials/basics/10_hello_world.html>
//! - <https://www.wikidata.org/wiki/Q21921428> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Crystal => "Crystal",
    versions: [Crystal1 => "Crystal 1"],
    extensions: [Cr => "cr"],
}

const CRYSTAL1: &str = concat!(r#"puts ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Crystal,
        extensions: &[FileExtension::Cr],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Crystal1,
            syntax: CRYSTAL1,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CRYSTAL1,
                    [
                        (TokenKind::Function, "puts"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };
