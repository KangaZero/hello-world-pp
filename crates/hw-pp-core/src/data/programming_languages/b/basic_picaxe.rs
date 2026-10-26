//! Hello-world snippet for `ProgrammingLanguageName::BasicPicaxe`.
//!
//! Imported from the Hello World Collection (`files/BASIC-PICAXE.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BASIC-PICAXE.htm>
//! - <http://helloworldcollection.de/#BASIC-PICAXE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BasicPicaxe => "BASIC-PICAXE",
    versions: [BasicPicaxe => "BASIC-PICAXE"],
    extensions: [],
}

const BASIC_PICAXE: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in PICAXE BASIC
Symbol TX_PIN = 0
Symbol TX_BAUD = N2400
SerOut TX_PIN, TX_BAUD, ( ""#,
    output_for_hello_world!(),
    r#"", CR, LF )"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BasicPicaxe,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BasicPicaxe,
            syntax: BASIC_PICAXE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BASIC_PICAXE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"' ",
                            output_for_hello_world!(),
                            r#" in PICAXE BASIC
Symbol TX_PIN = 0
Symbol TX_BAUD = N2400
SerOut TX_PIN, TX_BAUD, ( ""#,
                            output_for_hello_world!(),
                            r#"", CR, LF )"#
                        ),
                    )],
                )
            },
        }],
    };
