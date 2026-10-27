//! Hello-world snippet for `ProgrammingLanguageName::Nelua`.
//!
//! Imported from leachim6/hello-world (`n/Nelua.nelua` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nelua.nelua>
//! - <https://nelua.io/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Metaprogramming;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nelua => "Nelua",
    versions: [Nelua => "Nelua"],
    extensions: [Nelua => "nelua"],
}

const NELUA: &str = concat!(r"print '", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nelua,
        extensions: &[FileExtension::Nelua],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nelua,
            syntax: NELUA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NELUA,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                    ],
                )
            },
        }],
    };
