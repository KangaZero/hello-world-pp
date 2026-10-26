//! Hello-world snippet for `ProgrammingLanguageName::IclScl`.
//!
//! Imported from the Hello World Collection (`files/ICL&nbsp;SCL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ICL%26nbsp%3BSCL.txt>
//! - <http://helloworldcollection.de/#ICL%C2%A0SCL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: IclScl => "ICL SCL",
    versions: [IclScl => "ICL SCL"],
    extensions: [],
}

const ICL_SCL: &str = concat!(
    r"@ ",
    output_for_hello_world!(),
    r#" IN ICL SCL
BEGIN
    SEND_MESSAGE(""#,
    output_for_hello_world!(),
    r#"")
END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IclScl,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IclScl,
            syntax: ICL_SCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ICL_SCL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"@ ",
                            output_for_hello_world!(),
                            r#" IN ICL SCL
BEGIN
    SEND_MESSAGE(""#,
                            output_for_hello_world!(),
                            r#"")
END"#
                        ),
                    )],
                )
            },
        }],
    };
