//! Hello-world snippet for `ProgrammingLanguageName::Cardinal`.
//!
//! Imported from leachim6/hello-world (`c/Cardinal.cardinal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cardinal.cardinal>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cardinal => "Cardinal",
    versions: [Cardinal => "Cardinal"],
    extensions: [Cardinal => "cardinal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cardinal,
        extensions: &[FileExtension::Cardinal],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cardinal,
            syntax: concat!(r#"%""#, output_for_hello_world!(), r#"";x"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"%""#, output_for_hello_world!(), r#"";x"#),
            )],
        }],
    };
