//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80CpSlashM`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(Z80,&nbsp;CP&#47;M).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28Z80%2C%26nbsp%3BCP%26%2347%3BM%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(Z80,%C2%A0CP%2FM)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerZ80CpSlashM => "Assembler (Z80, CP/M)",
    versions: [AssemblerZ80CpSlashM => "Assembler (Z80, CP/M)"],
    extensions: [],
}

const ASSEMBLER_Z80_CP_SLASH_M: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" in Z80 Assembly for CP/M

BDOS    equ     05h
WRTLINE equ     09h
;
        org     0100h
        lxi     d,sHello
        mvi     c,WRTLINE
        call    BDOS
        ret
;
sHello  db      '",
    output_for_hello_world!(),
    r"$'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80CpSlashM,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80CpSlashM,
            syntax: ASSEMBLER_Z80_CP_SLASH_M,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_Z80_CP_SLASH_M,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" in Z80 Assembly for CP/M"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

BDOS    ",
                        ),
                        (TokenKind::Keyword, r"equ"),
                        (
                            TokenKind::Plain,
                            r"     05h
WRTLINE ",
                        ),
                        (TokenKind::Keyword, r"equ"),
                        (
                            TokenKind::Plain,
                            r"     09h
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"org"),
                        (
                            TokenKind::Plain,
                            r"     0100h
        ",
                        ),
                        (TokenKind::Keyword, r"lxi"),
                        (TokenKind::Plain, r"     d"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"sHello
        ",
                        ),
                        (TokenKind::Keyword, r"mvi"),
                        (TokenKind::Plain, r"     c"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"WRTLINE
        ",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"    BDOS
        ",
                        ),
                        (TokenKind::Keyword, r"ret"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
sHello  ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"      "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"$'"),
                        ),
                    ],
                )
            },
        }],
    };
