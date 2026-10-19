//! Hello-world snippet for `ProgrammingLanguageName::Broccoli`.
//!
//! Imported from leachim6/hello-world (`b/Broccoli.brocc` at `a152253`),
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
    language: Broccoli => "Broccoli",
    versions: [Broccoli => "Broccoli"],
    extensions: [Brocc => "brocc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Broccoli,
        extensions: &[FileExtension::Brocc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Broccoli,
            syntax: concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#),
            )],
        }],
    };
