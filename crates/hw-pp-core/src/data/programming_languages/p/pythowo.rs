//! Hello-world snippet for `ProgrammingLanguageName::Pythowo`.
//!
//! Imported from leachim6/hello-world (`p/pythOwO.pyowo` at `a152253`),
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
    language: Pythowo => "pythOwO",
    versions: [Pythowo => "pythOwO"],
    extensions: [Pyowo => "pyowo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pythowo,
        extensions: &[FileExtension::Pyowo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pythowo,
            syntax: concat!(r#"pwint(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"pwint(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
