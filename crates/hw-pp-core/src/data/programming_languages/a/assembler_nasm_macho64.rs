//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmMacho64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Macho64.asm` at `a152253`),
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
    language: AssemblerNasmMacho64 => "Assembler NASM Macho64",
    versions: [AssemblerNasmMacho64 => "Assembler NASM Macho64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmMacho64,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmMacho64,
            syntax: concat!(
                r#"global start

section .text
start:
    mov         rax, 0x2000004 
    mov         rdi, 1         
    mov         rsi, msg
    mov         rdx, msg.len
    syscall

    mov         rax, 0x2000001 
    mov         rdi, 0
    syscall

section .data
msg:    db      ""#,
                output_for_hello_world!(),
                r#"", 10
.len:   equ     $ - msg"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"global start

section .text
start:
    mov         rax, 0x2000004 
    mov         rdi, 1         
    mov         rsi, msg
    mov         rdx, msg.len
    syscall

    mov         rax, 0x2000001 
    mov         rdi, 0
    syscall

section .data
msg:    db      ""#,
                    output_for_hello_world!(),
                    r#"", 10
.len:   equ     $ - msg"#
                ),
            )],
        }],
    };
