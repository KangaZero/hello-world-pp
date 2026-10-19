//! Hello-world snippet for `ProgrammingLanguageName::SAlgol`.
//!
//! Imported from leachim6/hello-world (`s/S Algol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/S%20Algol>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SAlgol => "S Algol",
    versions: [SAlgol => "S Algol"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SAlgol,
        extensions: &[],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SAlgol,
            syntax: concat!(
                r#"write ""#,
                output_for_hello_world!(),
                r#""
?"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"write ""#,
                    output_for_hello_world!(),
                    r#""
?"#
                ),
            )],
        }],
    };
