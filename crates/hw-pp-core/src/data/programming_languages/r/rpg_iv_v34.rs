//! Hello-world snippet for `ProgrammingLanguageName::RpgIvV34`.
//!
//! Imported from the Hello World Collection (`files/RPG&nbsp;IV&nbsp;v3-4.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RPG%26nbsp%3BIV%26nbsp%3Bv3-4.htm>
//! - <http://helloworldcollection.de/#RPG%C2%A0IV%C2%A0v3-4>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: RpgIvV34 => "RPG IV v3-4",
    versions: [RpgIvV34 => "RPG IV v3-4"],
    extensions: [],
}

const RPG_IV_V34: &str = concat!(
    r"H* ",
    output_for_hello_world!(),
    r" in RPG IV versions 3 and 4

D msg             S             32    inz(*blank)
D wait            S              1

C                   eval      msg = '",
    output_for_hello_world!(),
    r"'

C     msg           dsply                   wait


C                   eval      *inlr = *on"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RpgIvV34,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RpgIvV34,
            syntax: RPG_IV_V34,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RPG_IV_V34,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"H* ",
                            output_for_hello_world!(),
                            r" in RPG IV versions 3 and 4

D msg             S             32    inz(*blank)
D wait            S              1

C                   eval      msg = '",
                            output_for_hello_world!(),
                            r"'

C     msg           dsply                   wait


C                   eval      *inlr = *on"
                        ),
                    )],
                )
            },
        }],
    };
