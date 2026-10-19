//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMasmWin32`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MASM Win32.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MASM%20Win32.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerMasmWin32 => "Assembler MASM Win32",
    versions: [AssemblerMasmWin32 => "Assembler MASM Win32"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMasmWin32,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMasmWin32,
            syntax: concat!(
                r"; ---------------------------------------------
; ",
                output_for_hello_world!(),
                r#" for Win32 Intel x86 Assembly
;
; by fruel (https://github.com/fruel)
; 13 June 2016
; ---------------------------------------------

.386
.model flat,stdcall
.stack 4096

EXTRN ExitProcess@4 : PROC
EXTRN GetStdHandle@4 : PROC
EXTRN WriteConsoleA@20 : PROC

.data
msg BYTE ""#,
                output_for_hello_world!(),
                r#"",0
bytesWritten DWORD ?

.code
main PROC
    push -11
    call GetStdHandle@4
 
    push 0   
    push OFFSET bytesWritten
    push LENGTHOF msg - 1
    push OFFSET msg   
    push eax     
    call WriteConsoleA@20

    push 0  
    call ExitProcess@4
main ENDP
END main"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ---------------------------------------------
; ",
                    output_for_hello_world!(),
                    r#" for Win32 Intel x86 Assembly
;
; by fruel (https://github.com/fruel)
; 13 June 2016
; ---------------------------------------------

.386
.model flat,stdcall
.stack 4096

EXTRN ExitProcess@4 : PROC
EXTRN GetStdHandle@4 : PROC
EXTRN WriteConsoleA@20 : PROC

.data
msg BYTE ""#,
                    output_for_hello_world!(),
                    r#"",0
bytesWritten DWORD ?

.code
main PROC
    push -11
    call GetStdHandle@4
 
    push 0   
    push OFFSET bytesWritten
    push LENGTHOF msg - 1
    push OFFSET msg   
    push eax     
    call WriteConsoleA@20

    push 0  
    call ExitProcess@4
main ENDP
END main"#
                ),
            )],
        }],
    };
