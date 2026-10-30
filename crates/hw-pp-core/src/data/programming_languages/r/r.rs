//! Hello-world snippet for `ProgrammingLanguageName::R`.
//!
//! Sources:
//! - <https://cran.r-project.org/doc/manuals/r-release/R-intro.html>
//! - <https://stat.ethz.ch/R-manual/R-devel/library/base/html/cat.html>
//! - <https://www.wikidata.org/wiki/Q206904> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Array, Functional, Imperative, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::R, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: R => "R",
    versions: [R4 => "R 4"],
    extensions: [R => "r"],
}

const R4: &str = concat!(r#"cat(""#, output_for_hello_world!(), r#"\n")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &[FileExtension::R],
        categories: &[Scripting],
        families: &[R],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Array,
            Reflective,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::R4,
            syntax: R4,
            tokenized_syntax: &const {
                checked_token_lengths(
                    R4,
                    [
                        (TokenKind::Function, "cat"),
                        (TokenKind::Plain, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };
