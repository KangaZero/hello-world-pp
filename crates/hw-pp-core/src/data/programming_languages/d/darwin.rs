//! Hello-world snippet for `ProgrammingLanguageName::Darwin`.
//!
//! Imported from leachim6/hello-world (`d/Darwin.drw` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Darwin.drw>

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
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Darwin,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"printf(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
