//! Hello-world snippet for `ProgrammingLanguageName::Lateralus`.
//!
//! Imported from leachim6/hello-world (PR #1802 `l/Lateralus.lat`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1802>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lateralus => "Lateralus",
    versions: [Lateralus => "Lateralus"],
    extensions: [Lat => "lat"],
}

const LATERALUS: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lateralus,
        extensions: &[FileExtension::Lat],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lateralus,
            syntax: LATERALUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LATERALUS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
