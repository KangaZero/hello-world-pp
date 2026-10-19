//! Hello-world snippet for `ProgrammingLanguageName::Glass`.
//!
//! Imported from leachim6/hello-world (`g/Glass.glass` at `a152253`),
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
    language: Glass => "Glass",
    versions: [Glass => "Glass"],
    extensions: [Glass => "glass"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Glass,
        extensions: &[FileExtension::Glass],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Glass,
            syntax: concat!(
                r#"+{M[m(_o)O!""#,
                output_for_hello_world!(),
                r#""(_o)o.?]}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"+{M[m(_o)O!""#,
                    output_for_hello_world!(),
                    r#""(_o)o.?]}"#
                ),
            )],
        }],
    };
