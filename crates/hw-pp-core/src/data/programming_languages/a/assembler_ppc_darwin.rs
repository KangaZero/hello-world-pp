//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPpcDarwin`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(PPC,&nbsp;Darwin).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28PPC%2C%26nbsp%3BDarwin%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(PPC,%C2%A0Darwin)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerPpcDarwin => "Assembler (PPC, Darwin)",
    versions: [AssemblerPpcDarwin => "Assembler (PPC, Darwin)"],
    extensions: [],
}

const ASSEMBLER_PPC_DARWIN: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Assembler for the Darwin Power-PC

.data
.cstring
.align 2
msg:
.asciz ""#,
    output_for_hello_world!(),
    r#"\n"
len = . - msg
.text
.align 2
.globl _start
_start:
li r0,4
li r3,1
lis r4,ha16(msg)
ori r4,r4,lo16(msg)
li r5,len
sc
li r0,1
li r3,0
sc"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPpcDarwin,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPpcDarwin,
            syntax: ASSEMBLER_PPC_DARWIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_PPC_DARWIN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r#" in Assembler for the Darwin Power-PC

.data
.cstring
.align 2
msg:
.asciz ""#,
                            output_for_hello_world!(),
                            r#"\n"
len = . - msg
.text
.align 2
.globl _start
_start:
li r0,4
li r3,1
lis r4,ha16(msg)
ori r4,r4,lo16(msg)
li r5,len
sc
li r0,1
li r3,0
sc"#
                        ),
                    )],
                )
            },
        }],
    };
