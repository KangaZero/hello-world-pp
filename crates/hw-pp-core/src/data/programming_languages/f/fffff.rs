//! Hello-world snippet for `ProgrammingLanguageName::Fffff`.
//!
//! Imported from leachim6/hello-world (PR #1483 `f/fffff.fffff`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1483>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fffff => "fffff",
    versions: [Fffff => "fffff"],
    extensions: [Fffff => "fffff"],
}

const FFFFF: &str = concat!(r#"""#, output_for_hello_world!(), r#"" println"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fffff,
        extensions: &[FileExtension::Fffff],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fffff,
            syntax: FFFFF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FFFFF,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" println"#),
                    )],
                )
            },
        }],
    };
