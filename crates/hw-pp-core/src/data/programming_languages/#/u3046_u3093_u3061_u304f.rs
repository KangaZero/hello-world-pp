//! Hello-world snippet for `ProgrammingLanguageName::U3046U3093U3061U304f`.
//!
//! Imported from leachim6/hello-world (`#/うんちく.unchk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E3%81%86%E3%82%93%E3%81%A1%E3%81%8F.unchk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: U3046U3093U3061U304f => "うんちく",
    versions: [U3046U3093U3061U304f => "うんちく"],
    extensions: [Unchk => "unchk"],
}

const U3046_U3093_U3061_U304F: &str = concat!(r"「", output_for_hello_world!(), r"」って書く。");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U3046U3093U3061U304f,
        extensions: &[FileExtension::Unchk],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U3046U3093U3061U304f,
            syntax: U3046_U3093_U3061_U304F,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U3046_U3093_U3061_U304F,
                    [(
                        TokenKind::Plain,
                        concat!(r"「", output_for_hello_world!(), r"」って書く。"),
                    )],
                )
            },
        }],
    };
