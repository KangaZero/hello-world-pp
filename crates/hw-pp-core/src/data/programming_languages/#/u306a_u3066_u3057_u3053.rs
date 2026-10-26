//! Hello-world snippet for `ProgrammingLanguageName::U306aU3066U3057U3053`.
//!
//! Imported from leachim6/hello-world (`#/なでしこ.nako` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E3%81%AA%E3%81%A7%E3%81%97%E3%81%93.nako>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: U306aU3066U3057U3053 => "なでしこ",
    versions: [U306aU3066U3057U3053 => "なでしこ"],
    extensions: [Nako => "nako"],
}

const U306A_U3066_U3057_U3053: &str = concat!(r"「", output_for_hello_world!(), r"」と表示");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U306aU3066U3057U3053,
        extensions: &[FileExtension::Nako],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U306aU3066U3057U3053,
            syntax: U306A_U3066_U3057_U3053,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U306A_U3066_U3057_U3053,
                    [(
                        TokenKind::Plain,
                        concat!(r"「", output_for_hello_world!(), r"」と表示"),
                    )],
                )
            },
        }],
    };
