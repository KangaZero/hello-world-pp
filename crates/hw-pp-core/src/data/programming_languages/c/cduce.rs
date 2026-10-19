//! Hello-world snippet for `ProgrammingLanguageName::Cduce`.
//!
//! Imported from leachim6/hello-world (`c/Cduce.cduce` at `a152253`),
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
    language: Cduce => "Cduce",
    versions: [Cduce => "Cduce"],
    extensions: [Cduce => "cduce"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cduce,
        extensions: &[FileExtension::Cduce],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cduce,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"";;"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"";;"#),
            )],
        }],
    };
