//! Hello-world snippet for `ProgrammingLanguageName::Quorum`.
//!
//! Imported from leachim6/hello-world (`q/Quorum.quorum` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/Quorum.quorum>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Quorum => "Quorum",
    versions: [Quorum => "Quorum"],
    extensions: [Quorum => "quorum"],
}

const QUORUM: &str = concat!(r#"output ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quorum,
        extensions: &[FileExtension::Quorum],
        categories: &[Educational],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quorum,
            syntax: QUORUM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUORUM,
                    [(
                        TokenKind::Plain,
                        concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
