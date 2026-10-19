//! Hello-world snippet for `ProgrammingLanguageName::Flix`.
//!
//! Imported from leachim6/hello-world (`f/Flix.flix` at `a152253`),
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
    language: Flix => "Flix",
    versions: [Flix => "Flix"],
    extensions: [Flix => "flix"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flix,
        extensions: &[FileExtension::Flix],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flix,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
