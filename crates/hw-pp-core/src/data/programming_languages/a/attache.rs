//! Hello-world snippet for `ProgrammingLanguageName::Attache`.
//!
//! Imported from leachim6/hello-world (`a/Attache.@` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Attache.%40>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Attache => "Attache",
    versions: [Attache => "Attache"],
    extensions: [At2 => "@"],
}

const ATTACHE: &str = concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Attache,
        extensions: &[FileExtension::At2],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Attache,
            syntax: ATTACHE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ATTACHE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Print[""#, output_for_hello_world!(), r#""]"#),
                    )],
                )
            },
        }],
    };
