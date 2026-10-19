//! Hello-world snippet for `ProgrammingLanguageName::Mouse`.
//!
//! Imported from leachim6/hello-world (`m/Mouse.mse` at `a152253`),
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
    language: Mouse => "Mouse",
    versions: [Mouse => "Mouse"],
    extensions: [Mse => "mse"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mouse,
        extensions: &[FileExtension::Mse],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mouse,
            syntax: concat!(
                r#"""#,
                output_for_hello_world!(),
                r#""
'! !'
$"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#""
'! !'
$"#
                ),
            )],
        }],
    };
