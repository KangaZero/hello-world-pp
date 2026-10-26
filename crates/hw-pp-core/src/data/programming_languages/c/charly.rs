//! Hello-world snippet for `ProgrammingLanguageName::Charly`.
//!
//! Imported from leachim6/hello-world (`c/Charly.ch` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Charly.ch>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Charly => "Charly",
    versions: [Charly => "Charly"],
    extensions: [Ch => "ch"],
}

const CHARLY: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Charly,
        extensions: &[FileExtension::Ch],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Charly,
            syntax: CHARLY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHARLY,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
