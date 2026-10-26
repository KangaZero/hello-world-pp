//! Hello-world snippet for `ProgrammingLanguageName::Tokay`.
//!
//! Imported from leachim6/hello-world (`t/Tokay.tok` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tokay.tok>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tokay => "Tokay",
    versions: [Tokay => "Tokay"],
    extensions: [Tok => "tok"],
}

const TOKAY: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tokay,
        extensions: &[FileExtension::Tok],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tokay,
            syntax: TOKAY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TOKAY,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
