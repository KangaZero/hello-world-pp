//! Hello-world snippet for `ProgrammingLanguageName::Hp41c`.
//!
//! Imported from leachim6/hello-world (`h/HP_41C` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HP_41C>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hp41c => "HP_41C",
    versions: [Hp41c => "HP_41C"],
    extensions: [],
}

const HP41C: &str = concat!(
    r#"01 LBL "HELLO"
02 ""#,
    output_for_hello_world!(),
    r#""
03 AVIEW"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hp41c,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hp41c,
            syntax: HP41C,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HP41C,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"01 LBL "HELLO"
02 ""#,
                            output_for_hello_world!(),
                            r#""
03 AVIEW"#
                        ),
                    )],
                )
            },
        }],
    };
