//! Hello-world snippet for `ProgrammingLanguageName::Aardio`.
//!
//! Imported from leachim6/hello-world (`a/aardio.aardio` at `a152253`),
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
    language: Aardio => "aardio",
    versions: [Aardio => "aardio"],
    extensions: [Aardio => "aardio"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aardio,
        extensions: &[FileExtension::Aardio],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aardio,
            syntax: concat!(
                r#"import console
console.log(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import console
console.log(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
