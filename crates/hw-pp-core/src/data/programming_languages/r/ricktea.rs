//! Hello-world snippet for `ProgrammingLanguageName::Ricktea`.
//!
//! Imported from leachim6/hello-world (`r/Ricktea.tea` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ricktea.tea>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ricktea => "Ricktea",
    versions: [Ricktea => "Ricktea"],
    extensions: [Tea => "tea"],
}

const RICKTEA: &str = concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ricktea,
        extensions: &[FileExtension::Tea],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ricktea,
            syntax: RICKTEA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RICKTEA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
