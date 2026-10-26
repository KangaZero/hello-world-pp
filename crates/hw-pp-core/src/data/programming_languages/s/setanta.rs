//! Hello-world snippet for `ProgrammingLanguageName::Setanta`.
//!
//! Imported from leachim6/hello-world (`s/Setanta.set` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Setanta.set>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Setanta => "Setanta",
    versions: [Setanta => "Setanta"],
    extensions: [Set => "set"],
}

const SETANTA: &str = concat!(r#"scríobh(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Setanta,
        extensions: &[FileExtension::Set],
        categories: &[Educational],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Setanta,
            syntax: SETANTA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SETANTA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"scríobh(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
