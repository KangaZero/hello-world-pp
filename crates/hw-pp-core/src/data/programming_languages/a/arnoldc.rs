//! Hello-world snippet for `ProgrammingLanguageName::Arnoldc`.
//!
//! Imported from leachim6/hello-world (`a/ArnoldC.arnoldc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ArnoldC.arnoldc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Arnoldc => "ArnoldC",
    versions: [Arnoldc => "ArnoldC"],
    extensions: [Arnoldc => "arnoldc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arnoldc,
        extensions: &[FileExtension::Arnoldc],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arnoldc,
            syntax: concat!(
                r#"IT'S SHOWTIME
TALK TO THE HAND ""#,
                output_for_hello_world!(),
                r#""
YOU HAVE BEEN TERMINATED"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"IT'S SHOWTIME
TALK TO THE HAND ""#,
                    output_for_hello_world!(),
                    r#""
YOU HAVE BEEN TERMINATED"#
                ),
            )],
        }],
    };
