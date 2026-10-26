//! Hello-world snippet for `ProgrammingLanguageName::Mamash`.
//!
//! Imported from the Hello World Collection (`files/MAMASH.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MAMASH.txt>
//! - <http://helloworldcollection.de/#MAMASH>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mamash => "MAMASH",
    versions: [Mamash => "MAMASH"],
    extensions: [],
}

const MAMASH: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r" in MAMASH */

TQWD LGYEA NXKIA HELLO_WORLD )1(
 
DWLH CXBZ_YKX
 
ID& HELLO_WORLD YED '",
    output_for_hello_world!(),
    r"' ."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mamash,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mamash,
            syntax: MAMASH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAMASH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in MAMASH */

TQWD LGYEA NXKIA HELLO_WORLD )1(
 
DWLH CXBZ_YKX
 
ID& HELLO_WORLD YED '",
                            output_for_hello_world!(),
                            r"' ."
                        ),
                    )],
                )
            },
        }],
    };
