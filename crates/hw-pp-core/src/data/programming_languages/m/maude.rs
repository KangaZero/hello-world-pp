//! Hello-world snippet for `ProgrammingLanguageName::Maude`.
//!
//! Imported from leachim6/hello-world (PR #1770 `m/Maude.maude`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1770>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Maude => "Maude",
    versions: [Maude => "Maude"],
    extensions: [Maude => "maude"],
}

const MAUDE: &str = concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maude,
        extensions: &[FileExtension::Maude],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maude,
            syntax: MAUDE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAUDE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#),
                    )],
                )
            },
        }],
    };
