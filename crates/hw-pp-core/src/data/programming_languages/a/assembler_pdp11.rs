//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp11`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(PDP-11)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28PDP-11%29>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(PDP-11)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerPdp11 => "Assembler (PDP-11)",
    versions: [AssemblerPdp11 => "Assembler (PDP-11)"],
    extensions: [],
}

const ASSEMBLER_PDP11: &str = concat!(
    r";       ",
    output_for_hello_world!(),
    r" in Assembler for the DEC PDP-11 with the
;	RSX-11M-PLUS operating system
;
        .title Hello
        .ident /V0001A/
        .mcall qiow$s, exit$s
        .psect $code,ro,i
start:  qiow$s #5,#5,,,,<#str, #len, #40>
        exit$s
        .psect $data,ro,d
str:    .ascii / ",
    output_for_hello_world!(),
    r"/
        len=.-str
        .end start"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp11,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp11,
            syntax: ASSEMBLER_PDP11,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_PDP11,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r";       ",
                                output_for_hello_world!(),
                                r" in Assembler for the DEC PDP-11 with the"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";	RSX-11M-PLUS operating system"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Macro, r".title"),
                        (
                            TokenKind::Plain,
                            r" Hello
        ",
                        ),
                        (TokenKind::Macro, r".ident"),
                        (
                            TokenKind::Plain,
                            r" /V0001A/
        ",
                        ),
                        (TokenKind::Macro, r".mcall"),
                        (TokenKind::Plain, r" qiow$s"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" exit$s
        ",
                        ),
                        (TokenKind::Macro, r".psect"),
                        (TokenKind::Plain, r" $code"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"ro"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"i
start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"  qiow$s #"),
                        (TokenKind::Number, r"5"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"5"),
                        (TokenKind::Punctuation, r",,,,"),
                        (TokenKind::Plain, r"<#str"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" #len"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"40"),
                        (
                            TokenKind::Plain,
                            r">
        ",
                        ),
                        (TokenKind::Keyword, r"exit"),
                        (
                            TokenKind::Plain,
                            r"$s
        ",
                        ),
                        (TokenKind::Macro, r".psect"),
                        (TokenKind::Plain, r" $data"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"ro"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"d
str",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"    "),
                        (TokenKind::Macro, r".ascii"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" / ",
                                output_for_hello_world!(),
                                r"/
        "
                            ),
                        ),
                        (TokenKind::Keyword, r"len"),
                        (TokenKind::Plain, r"="),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"-str
        ",
                        ),
                        (TokenKind::Macro, r".end"),
                        (TokenKind::Plain, r" start"),
                    ],
                )
            },
        }],
    };
