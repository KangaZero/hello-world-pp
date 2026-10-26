//! Hello-world snippet for `ProgrammingLanguageName::Ride`.
//!
//! Imported from leachim6/hello-world (`r/Ride.ride` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ride.ride>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ride => "Ride",
    versions: [Ride => "Ride"],
    extensions: [Ride => "ride"],
}

const RIDE: &str = concat!(r#"throw(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ride,
        extensions: &[FileExtension::Ride],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ride,
            syntax: RIDE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RIDE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"throw(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
