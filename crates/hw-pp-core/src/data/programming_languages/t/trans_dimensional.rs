//! Hello-world snippet for `ProgrammingLanguageName::TransDimensional`.
//!
//! Imported from leachim6/hello-world (`t/Trans-dimensional` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Trans-dimensional>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: TransDimensional => "Trans-dimensional",
    versions: [TransDimensional => "Trans-dimensional"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TransDimensional,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TransDimensional,
            syntax: concat!(
                r#"dimension X;
put [X:1] -> ""#,
                output_for_hello_world!(),
                r#"";
output get [X:1];"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"dimension X;
put [X:1] -> ""#,
                    output_for_hello_world!(),
                    r#"";
output get [X:1];"#
                ),
            )],
        }],
    };
