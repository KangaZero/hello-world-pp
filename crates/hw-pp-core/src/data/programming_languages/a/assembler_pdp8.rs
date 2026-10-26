//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp8`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(PDP-8).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28PDP-8%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(PDP-8)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerPdp8 => "Assembler (PDP-8)",
    versions: [AssemblerPdp8 => "Assembler (PDP-8)"],
    extensions: [],
}

const ASSEMBLER_PDP8: &str = concat!(
    r"/ ",
    output_for_hello_world!(),
    r" in Assembler for the DEC PDP-8
*200
hello,    cla cll
        tls            / tls to set printer flag.
        tad charac    / set up index register
        dca ir1        / for getting characters.
        tad m6        / set up counter for
        dca count    / typing characters.
next,    tad i ir1    / get a character.
        jms type    / type it.
        isz count    / done yet?
        jmp next    / no: type another.
        hlt

type,    0            / type subroutine
        tsf
        jmp .-1
        tls
        cla
        jmp i type
charac,    .            / used as initial value of ir1
        310 / H
        305 / E
        314 / L
        314 / L
        317 / O
        254 / ,
        240 /
        327 / W
        317 / O
        322 / R
        314 / L
        304 / D
        241 / !
m6,        -15
count,    0
ir1 = 10
$"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp8,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp8,
            syntax: ASSEMBLER_PDP8,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_PDP8,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/ ",
                            output_for_hello_world!(),
                            r" in Assembler for the DEC PDP-8
*200
hello,    cla cll
        tls            / tls to set printer flag.
        tad charac    / set up index register
        dca ir1        / for getting characters.
        tad m6        / set up counter for
        dca count    / typing characters.
next,    tad i ir1    / get a character.
        jms type    / type it.
        isz count    / done yet?
        jmp next    / no: type another.
        hlt

type,    0            / type subroutine
        tsf
        jmp .-1
        tls
        cla
        jmp i type
charac,    .            / used as initial value of ir1
        310 / H
        305 / E
        314 / L
        314 / L
        317 / O
        254 / ,
        240 /
        327 / W
        317 / O
        322 / R
        314 / L
        304 / D
        241 / !
m6,        -15
count,    0
ir1 = 10
$"
                        ),
                    )],
                )
            },
        }],
    };
