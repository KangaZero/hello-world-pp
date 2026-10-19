//! Hello-world snippet for `ProgrammingLanguageName::Hp41c`.
//!
//! Imported from leachim6/hello-world (`h/HP_41C` at `a152253`),
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
    language: Hp41c => "HP_41C",
    versions: [Hp41c => "HP_41C"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hp41c,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hp41c,
            syntax: concat!(
                r#"01 LBL "HELLO"
02 ""#,
                output_for_hello_world!(),
                r#""
03 AVIEW"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"01 LBL "HELLO"
02 ""#,
                    output_for_hello_world!(),
                    r#""
03 AVIEW"#
                ),
            )],
        }],
    };
