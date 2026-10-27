//! Hello-world snippet for `ProgrammingLanguageName::Backshift`.
//!
//! Imported from leachim6/hello-world (`b/Backshift` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Backshift>
//! - <https://esolangs.org/wiki/Backshift> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::QueueBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Backshift => "Backshift",
    versions: [Backshift => "Backshift"],
    extensions: [],
}

const BACKSHIFT: &str = concat!(r" ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Backshift,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Backshift,
            syntax: BACKSHIFT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BACKSHIFT,
                    [(TokenKind::Plain, concat!(r" ", output_for_hello_world!()))],
                )
            },
        }],
    };
