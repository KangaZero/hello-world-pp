//! Hello-world snippet for `ProgrammingLanguageName::Rpl`.
//!
//! Imported from the Hello World Collection (`files/RPL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RPL.txt>
//! - <http://helloworldcollection.de/#RPL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rpl => "RPL",
    versions: [Rpl => "RPL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rpl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rpl,
            syntax: concat!(
                output_for_hello_world!(),
                r#" in RPL for the HP-28, HP-48, HP-49 and HP-50 series pocket calculators. No comments possible.

<<
    ""#,
                output_for_hello_world!(),
                r#""
    1 DISP
    60 FREEZE
>>"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r#" in RPL for the HP-28, HP-48, HP-49 and HP-50 series pocket calculators. No comments possible.

<<
    ""#,
                    output_for_hello_world!(),
                    r#""
    1 DISP
    60 FREEZE
>>"#
                ),
            )],
        }],
    };
