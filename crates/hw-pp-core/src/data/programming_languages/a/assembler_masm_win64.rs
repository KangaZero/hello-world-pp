//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMasmWin64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MASM Win64.asm` at `a152253`),
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
    language: AssemblerMasmWin64 => "Assembler MASM Win64",
    versions: [AssemblerMasmWin64 => "Assembler MASM Win64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMasmWin64,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMasmWin64,
            syntax: concat!(
                r"; ---------------------------------------------
; ",
                output_for_hello_world!(),
                r#" for Win64 Intel x64 Assembly
;
; by fruel (https://github.com/fruel)
; 13 June 2016
; ---------------------------------------------

GetStdHandle PROTO
ExitProcess PROTO
WriteConsoleA PROTO

.data
msg BYTE ""#,
                output_for_hello_world!(),
                r#"",0
bytesWritten DWORD ?

.code
main PROC
    sub rsp, 5 * 8  

    mov rcx, -11           
    call GetStdHandle

    mov  rcx, rax       
    lea  rdx, msg    
    mov  r8, LENGTHOF msg - 1
    lea  r9, bytesWritten  
    mov  QWORD PTR [rsp + 4 * SIZEOF QWORD], 0
    call WriteConsoleA

    mov rcx, 0      
    call ExitProcess
main ENDP
END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ---------------------------------------------
; ",
                    output_for_hello_world!(),
                    r#" for Win64 Intel x64 Assembly
;
; by fruel (https://github.com/fruel)
; 13 June 2016
; ---------------------------------------------

GetStdHandle PROTO
ExitProcess PROTO
WriteConsoleA PROTO

.data
msg BYTE ""#,
                    output_for_hello_world!(),
                    r#"",0
bytesWritten DWORD ?

.code
main PROC
    sub rsp, 5 * 8  

    mov rcx, -11           
    call GetStdHandle

    mov  rcx, rax       
    lea  rdx, msg    
    mov  r8, LENGTHOF msg - 1
    lea  r9, bytesWritten  
    mov  QWORD PTR [rsp + 4 * SIZEOF QWORD], 0
    call WriteConsoleA

    mov rcx, 0      
    call ExitProcess
main ENDP
END"#
                ),
            )],
        }],
    };
