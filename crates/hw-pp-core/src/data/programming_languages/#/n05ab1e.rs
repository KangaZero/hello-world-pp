//! Hello-world snippet for `ProgrammingLanguageName::N05ab1e`.
//!
//! Imported from leachim6/hello-world (`#/05AB1E` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/05AB1E>
//! - <https://esolangs.org/wiki/05AB1E> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: N05ab1e => "05AB1E",
    versions: [N05ab1e => "05AB1E"],
    extensions: [],
}

const N05AB1E: &str = concat!(r#"""#, output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N05ab1e,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N05ab1e,
            syntax: N05AB1E,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N05AB1E,
                    [(TokenKind::Plain, concat!(r#"""#, output_for_hello_world!()))],
                )
            },
        }],
    };
