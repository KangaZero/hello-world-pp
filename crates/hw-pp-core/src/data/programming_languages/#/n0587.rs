//! Hello-world snippet for `ProgrammingLanguageName::N0587`.
//!
//! Imported from leachim6/hello-world (`#/0587` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/0587>
//! - <https://esolangs.org/wiki/0587> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N0587 => "0587",
    versions: [N0587 => "0587"],
    extensions: [],
}

const N0587: &str = concat!(r"04(", output_for_hello_world!(), r")");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0587,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0587,
            syntax: N0587,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N0587,
                    [(
                        TokenKind::Plain,
                        concat!(r"04(", output_for_hello_world!(), r")"),
                    )],
                )
            },
        }],
    };
