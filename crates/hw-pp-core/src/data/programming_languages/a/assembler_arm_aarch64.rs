//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArmAarch64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler ARM aarch64.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerArmAarch64 => "Assembler ARM aarch64",
    versions: [AssemblerArmAarch64 => "Assembler ARM aarch64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArmAarch64,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArmAarch64,
            syntax: concat!(
                r#".data

msg:
    .ascii        ""#,
                output_for_hello_world!(),
                r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     x0, #1
    ldr     x1, =msg
    ldr     x2, =len
    mov     w8, #64
    svc     #0

    mov     x0, #0
    mov     w8, #93
    svc     #0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#".data

msg:
    .ascii        ""#,
                    output_for_hello_world!(),
                    r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     x0, #1
    ldr     x1, =msg
    ldr     x2, =len
    mov     w8, #64
    svc     #0

    mov     x0, #0
    mov     w8, #93
    svc     #0"#
                ),
            )],
        }],
    };
