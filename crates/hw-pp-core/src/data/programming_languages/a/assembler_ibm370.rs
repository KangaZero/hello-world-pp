//! Hello-world snippet for `ProgrammingLanguageName::AssemblerIbm370`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(IBM&nbsp;370).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28IBM%26nbsp%3B370%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(IBM%C2%A0370)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerIbm370 => "Assembler (IBM 370)",
    versions: [AssemblerIbm370 => "Assembler (IBM 370)"],
    extensions: [],
}

const ASSEMBLER_IBM370: &str = concat!(
    r"ITLE '",
    output_for_hello_world!(),
    r" for IBM Assembler/370 (VM/CMS)'
HELLO    START
BALR  12,0
USING *,12
*
WRTERM '",
    output_for_hello_world!(),
    r"'
*
SR    15,15
BR    14
*
END   HELLO"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerIbm370,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerIbm370,
            syntax: ASSEMBLER_IBM370,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_IBM370,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"ITLE '",
                            output_for_hello_world!(),
                            r" for IBM Assembler/370 (VM/CMS)'
HELLO    START
BALR  12,0
USING *,12
*
WRTERM '",
                            output_for_hello_world!(),
                            r"'
*
SR    15,15
BR    14
*
END   HELLO"
                        ),
                    )],
                )
            },
        }],
    };
