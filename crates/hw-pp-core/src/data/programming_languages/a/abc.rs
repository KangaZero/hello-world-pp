//! Hello-world snippet for `ProgrammingLanguageName::Abc`.
//!
//! Imported from leachim6/hello-world (`a/ABC.abc` at `a152253`),
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
    language: Abc => "ABC",
    versions: [Abc => "ABC"],
    extensions: [Abc => "abc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Abc,
        extensions: &[FileExtension::Abc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Abc,
            syntax: concat!(r#"WRITE ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"WRITE ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
