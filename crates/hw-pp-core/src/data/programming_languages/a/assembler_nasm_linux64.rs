//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmLinux64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Linux64.asm` at `a152253`),
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
    language: AssemblerNasmLinux64 => "Assembler NASM Linux64",
    versions: [AssemblerNasmLinux64 => "Assembler NASM Linux64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmLinux64,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmLinux64,
            syntax: concat!(
                r#"section .rodata
    msg db ""#,
                output_for_hello_world!(),
                r#"", 0xA     
    len equ $- msg           
section	.text
    global _start            

_start:
	mov	eax, 1        
	mov	edi, eax             
	mov	esi, msg           
    mov edx, len
	syscall                  

    mov eax, 60                 
	xor	edi, edi              
	syscall              "#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"section .rodata
    msg db ""#,
                    output_for_hello_world!(),
                    r#"", 0xA     
    len equ $- msg           
section	.text
    global _start            

_start:
	mov	eax, 1        
	mov	edi, eax             
	mov	esi, msg           
    mov edx, len
	syscall                  

    mov eax, 60                 
	xor	edi, edi              
	syscall              "#
                ),
            )],
        }],
    };
