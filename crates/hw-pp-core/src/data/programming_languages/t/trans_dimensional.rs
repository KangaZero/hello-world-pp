//! Hello-world snippet for `ProgrammingLanguageName::TransDimensional`.
//!
//! Imported from leachim6/hello-world (`t/Trans-dimensional` at `a152253`),
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
    language: TransDimensional => "Trans-dimensional",
    versions: [TransDimensional => "Trans-dimensional"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TransDimensional,
        extensions: &[],
        categories: &[],
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
