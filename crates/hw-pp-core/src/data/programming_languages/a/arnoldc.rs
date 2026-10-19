//! Hello-world snippet for `ProgrammingLanguageName::Arnoldc`.
//!
//! Imported from leachim6/hello-world (`a/ArnoldC.arnoldc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
