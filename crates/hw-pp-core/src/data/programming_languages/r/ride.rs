//! Hello-world snippet for `ProgrammingLanguageName::Ride`.
//!
//! Imported from leachim6/hello-world (`r/Ride.ride` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ride.ride>
//! - <https://docs.waves.tech/en/ride/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

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
        paradigms: &[Functional],
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
