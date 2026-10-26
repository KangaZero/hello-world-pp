//! Hello-world snippet for `ProgrammingLanguageName::Pankti`.
//!
//! Imported from leachim6/hello-world (PR #1562 `p/Pankti.pank`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1562>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pankti => "Pankti",
    versions: [Pankti => "Pankti"],
    extensions: [Pank => "pank"],
}

const PANKTI: &str = concat!(r#"দেখাও(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pankti,
        extensions: &[FileExtension::Pank],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pankti,
            syntax: PANKTI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PANKTI,
                    [(
                        TokenKind::Plain,
                        concat!(r#"দেখাও(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
