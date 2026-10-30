//! Hello-world snippet for `ProgrammingLanguageName::LimsBasic`.
//!
//! Imported from the Hello World Collection (`files/LIMS&nbsp;Basic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LIMS%26nbsp%3BBasic.txt>
//! - <http://helloworldcollection.de/#LIMS%C2%A0Basic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: LimsBasic => "LIMS Basic",
    versions: [LimsBasic => "LIMS Basic"],
    extensions: [],
}

const LIMS_BASIC: &str = concat!(
    r"'",
    output_for_hello_world!(),
    r#" in LIMS Basic
msgbox(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LimsBasic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LimsBasic,
            syntax: LIMS_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIMS_BASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"'",
                            output_for_hello_world!(),
                            r#" in LIMS Basic
msgbox(""#,
                            output_for_hello_world!(),
                            r#"")"#
                        ),
                    )],
                )
            },
        }],
    };
