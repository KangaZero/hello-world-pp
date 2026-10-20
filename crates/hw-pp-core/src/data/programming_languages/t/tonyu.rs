//! Hello-world snippet for `ProgrammingLanguageName::Tonyu`.
//!
//! Imported from leachim6/hello-world (`t/Tonyu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tonyu>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tonyu => "Tonyu",
    versions: [Tonyu => "Tonyu"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tonyu,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tonyu,
            syntax: concat!(
                r#"new Label{x:50, y:20, text:""#,
                output_for_hello_world!(),
                r#""};"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"new Label{x:50, y:20, text:""#,
                    output_for_hello_world!(),
                    r#""};"#
                ),
            )],
        }],
    };
