//! Hello-world snippet for `ProgrammingLanguageName::Uff1fDollar51Eq`.
//!
//! Imported from leachim6/hello-world (`#/？$51=` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%EF%BC%9F%2451%3D>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Uff1fDollar51Eq => "？$51=",
    versions: [Uff1fDollar51Eq => "？$51="],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Uff1fDollar51Eq,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Uff1fDollar51Eq,
            syntax: concat!(r"$? 5 ", output_for_hello_world!(), r" 5"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"$? 5 ", output_for_hello_world!(), r" 5"),
            )],
        }],
    };
