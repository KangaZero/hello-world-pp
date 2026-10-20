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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Maude => "Maude",
    versions: [Maude => "Maude"],
    extensions: [Maude => "maude"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maude,
        extensions: &[FileExtension::Maude],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maude,
            syntax: concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#),
            )],
        }],
    };
