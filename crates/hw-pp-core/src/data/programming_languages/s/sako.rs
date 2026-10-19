//! Hello-world snippet for `ProgrammingLanguageName::Sako`.
//!
//! Imported from leachim6/hello-world (PR #1542 `s/SAKO.sako`),
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
    language: Sako => "SAKO",
    versions: [Sako => "SAKO"],
    extensions: [Sako => "sako"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sako,
        extensions: &[FileExtension::Sako],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sako,
            syntax: concat!(
                r"TEKST:
",
                output_for_hello_world!(),
                r"
STOP NASTEPNY
KONIEC"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"TEKST:
",
                    output_for_hello_world!(),
                    r"
STOP NASTEPNY
KONIEC"
                ),
            )],
        }],
    };
