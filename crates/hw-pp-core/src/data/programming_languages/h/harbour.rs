//! Hello-world snippet for `ProgrammingLanguageName::Harbour`.
//!
//! Imported from leachim6/hello-world (`h/Harbour.prg` at `a152253`),
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
    language: Harbour => "Harbour",
    versions: [Harbour => "Harbour"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Harbour,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Harbour,
            syntax: concat!(
                r#"PROCEDURE Main()

   ? ""#,
                output_for_hello_world!(),
                r#""

   RETURN"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"PROCEDURE Main()

   ? ""#,
                    output_for_hello_world!(),
                    r#""

   RETURN"#
                ),
            )],
        }],
    };
