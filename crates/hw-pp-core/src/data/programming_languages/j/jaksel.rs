//! Hello-world snippet for `ProgrammingLanguageName::Jaksel`.
//!
//! Imported from leachim6/hello-world (PR #1638 `j/Jaksel.jaksel`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1638>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jaksel => "Jaksel",
    versions: [Jaksel => "Jaksel"],
    extensions: [Jaksel => "jaksel"],
}

const JAKSEL: &str = concat!(r#"spill ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jaksel,
        extensions: &[FileExtension::Jaksel],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jaksel,
            syntax: JAKSEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAKSEL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"spill ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
