//! Hello-world snippet for `ProgrammingLanguageName::Mad`.
//!
//! Imported from the Hello World Collection (`files/MAD.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MAD.htm>
//! - <http://helloworldcollection.de/#MAD>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Algol,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mad => "MAD",
    versions: [Mad => "MAD"],
    extensions: [],
}

const MAD: &str = concat!(
    r"         R ",
    output_for_hello_world!(),
    r" in MAD

PRINT FORMAT HELLOW
VECTOR VALUES HELLOW=$13h0",
    output_for_hello_world!(),
    r"*$
END OF PROGRAM"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mad,
        extensions: &[],
        categories: &[],
        families: &[Algol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mad,
            syntax: MAD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"         R ",
                            output_for_hello_world!(),
                            r" in MAD

PRINT FORMAT HELLOW
VECTOR VALUES HELLOW=$13h0",
                            output_for_hello_world!(),
                            r"*$
END OF PROGRAM"
                        ),
                    )],
                )
            },
        }],
    };
