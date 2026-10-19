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
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerPdp11 => "Assembler (PDP-11)",
    versions: [AssemblerPdp11 => "Assembler (PDP-11)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp11,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp11,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
