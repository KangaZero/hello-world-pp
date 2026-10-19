//! Hello-world snippet for `ProgrammingLanguageName::Broccoli`.
//!
//! Imported from leachim6/hello-world (`b/Broccoli.brocc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Broccoli.brocc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Broccoli,
            syntax: concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#),
            )],
        }],
    };
