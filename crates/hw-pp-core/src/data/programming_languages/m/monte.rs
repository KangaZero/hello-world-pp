//! Hello-world snippet for `ProgrammingLanguageName::Monte`.
//!
//! Imported from leachim6/hello-world (`m/Monte.mt` at `a152253`),
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
    language: Monte => "Monte",
    versions: [Monte => "Monte"],
    extensions: [Mt => "mt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Monte,
        extensions: &[FileExtension::Mt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Monte,
            syntax: concat!(r#"traceln(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"traceln(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
