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
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Quorum => "Quorum",
    versions: [Quorum => "Quorum"],
    extensions: [Quorum => "quorum"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quorum,
        extensions: &[FileExtension::Quorum],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quorum,
            syntax: concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
