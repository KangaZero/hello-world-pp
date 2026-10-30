//! Hello-world snippet for `ProgrammingLanguageName::Blc`.
//!
//! Imported from leachim6/hello-world (`b/BLC.Blc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BLC.Blc>
//! - <https://esolangs.org/wiki/Binary_lambda_calculus> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Blc => "BLC",
    versions: [Blc => "BLC"],
    extensions: [Blc => "Blc"],
}

const BLC: &str = concat!(r" ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blc,
        extensions: &[FileExtension::Blc],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blc,
            syntax: BLC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLC,
                    [(TokenKind::Plain, concat!(r" ", output_for_hello_world!()))],
                )
            },
        }],
    };
