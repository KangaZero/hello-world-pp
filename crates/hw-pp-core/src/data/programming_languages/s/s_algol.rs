//! Hello-world snippet for `ProgrammingLanguageName::SAlgol`.
//!
//! Imported from leachim6/hello-world (`s/S Algol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
