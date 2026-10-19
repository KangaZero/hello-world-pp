//! Hello-world snippet for `ProgrammingLanguageName::Darwin`.
//!
//! Imported from leachim6/hello-world (`d/Darwin.drw` at `a152253`),
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
    language: Darwin => "Darwin",
    versions: [Darwin => "Darwin"],
    extensions: [Drw => "drw"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Darwin,
        extensions: &[FileExtension::Drw],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Darwin,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"printf(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
