//! Hello-world snippet for `ProgrammingLanguageName::Rhai`.
//!
//! Imported from leachim6/hello-world (PR #1593 `r/Rhai.rhai`),
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
    language: Rhai => "Rhai",
    versions: [Rhai => "Rhai"],
    extensions: [Rhai => "rhai"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rhai,
        extensions: &[FileExtension::Rhai],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rhai,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
