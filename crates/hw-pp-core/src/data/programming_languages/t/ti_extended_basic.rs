//! Hello-world snippet for `ProgrammingLanguageName::TiExtendedBasic`.
//!
//! Imported from the Hello World Collection (`files/TI&nbsp;Extended&nbsp;BASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TI%26nbsp%3BExtended%26nbsp%3BBASIC.txt>
//! - <http://helloworldcollection.de/#TI%C2%A0Extended%C2%A0BASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TiExtendedBasic => "TI Extended BASIC",
    versions: [TiExtendedBasic => "TI Extended BASIC"],
    extensions: [],
}

const TI_EXTENDED_BASIC: &str = concat!(
    r"10 REM ",
    output_for_hello_world!(),
    r#" in Extended BASIC
20 REM for the TI99 series computer
100 CALL CLEAR :: DISPLAY AT(10,5):""#,
    output_for_hello_world!(),
    r#"" :: ACCEPT AT(20,4):A$"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TiExtendedBasic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TiExtendedBasic,
            syntax: TI_EXTENDED_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TI_EXTENDED_BASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"10 REM ",
                            output_for_hello_world!(),
                            r#" in Extended BASIC
20 REM for the TI99 series computer
100 CALL CLEAR :: DISPLAY AT(10,5):""#,
                            output_for_hello_world!(),
                            r#"" :: ACCEPT AT(20,4):A$"#
                        ),
                    )],
                )
            },
        }],
    };
