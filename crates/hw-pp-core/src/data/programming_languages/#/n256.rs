//! Hello-world snippet for `ProgrammingLanguageName::N256`.
//!
//! Imported from leachim6/hello-world (`#/256.256` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/256.256>
//! - <https://esolangs.org/wiki/256> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: N256 => "256",
    versions: [N256 => "256"],
    extensions: [E256 => "256"],
}

const N256: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N256,
        extensions: &[FileExtension::E256],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N256,
            syntax: N256,
            tokenized_syntax: &const {
                checked_token_lengths(N256, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
