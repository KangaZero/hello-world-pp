//! Hello-world snippet for `ProgrammingLanguageName::Nim`.
//!
//! Sources:
//! - <https://nim-lang.org/docs/tut1.html>
//! - <https://nim-lang.org/blog/2023/08/01/nim-v20-released.html>
//! - <https://www.wikidata.org/wiki/Q20080327> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Functional, Imperative, Metaprogramming, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Python, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nim => "Nim",
    versions: [Nim2 => "Nim 2"],
    extensions: [Nim => "nim"],
}

const NIM2: &str = concat!(r#"echo ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nim,
        extensions: &[FileExtension::Nim],
        categories: &[Compiled],
        families: &[Python],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Concurrent,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nim2,
            syntax: NIM2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NIM2,
                    [
                        (TokenKind::Function, "echo"),
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
