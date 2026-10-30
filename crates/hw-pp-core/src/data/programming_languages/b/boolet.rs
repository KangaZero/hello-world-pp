//! Hello-world snippet for `ProgrammingLanguageName::Boolet`.
//!
//! Imported from leachim6/hello-world (`b/Boolet` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Boolet>
//! - <https://esolangs.org/wiki/Boolet> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Boolet => "Boolet",
    versions: [Boolet => "Boolet"],
    extensions: [],
}

const BOOLET: &str = concat!(r"(", output_for_hello_world!(), r")");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Boolet,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Boolet,
            syntax: BOOLET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BOOLET,
                    [(
                        TokenKind::Plain,
                        concat!(r"(", output_for_hello_world!(), r")"),
                    )],
                )
            },
        }],
    };
