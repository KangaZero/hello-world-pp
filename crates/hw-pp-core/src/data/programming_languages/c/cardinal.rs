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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cardinal => "Cardinal",
    versions: [Cardinal => "Cardinal"],
    extensions: [Cardinal => "cardinal"],
}

const CARDINAL: &str = concat!(r#"%""#, output_for_hello_world!(), r#"";x"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cardinal,
        extensions: &[FileExtension::Cardinal],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cardinal,
            syntax: CARDINAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CARDINAL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"%""#, output_for_hello_world!(), r#"";x"#),
                    )],
                )
            },
        }],
    };
