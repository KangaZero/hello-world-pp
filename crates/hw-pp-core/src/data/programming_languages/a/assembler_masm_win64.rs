//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMasmWin64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MASM Win64.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MASM%20Win64.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Assembly],
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
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    r"; ---------------------------------------------",
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (
                    TokenKind::Comment,
                    concat!(
                        r"; ",
                        output_for_hello_world!(),
                        r" for Win64 Intel x64 Assembly"
                    ),
                ),
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
                (TokenKind::Comment, r"; by fruel (https://github.com/fruel)"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Comment, r"; 13 June 2016"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (
                    TokenKind::Comment,
                    r"; ---------------------------------------------",
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"GetStdHandle"),
                (
                    TokenKind::Plain,
                    r" PROTO
",
                ),
                (TokenKind::Keyword, r"ExitProcess"),
                (
                    TokenKind::Plain,
                    r" PROTO
",
                ),
                (TokenKind::Keyword, r"WriteConsoleA"),
                (
                    TokenKind::Plain,
                    r" PROTO

",
                ),
                (TokenKind::Macro, r".data"),
                (
                    TokenKind::Plain,
                    r"
msg ",
                ),
                (TokenKind::Keyword, r"BYTE"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
bytesWritten ",
                ),
                (TokenKind::Keyword, r"DWORD"),
                (
                    TokenKind::Plain,
                    r" ?

",
                ),
                (TokenKind::Macro, r".code"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"main"),
                (
                    TokenKind::Plain,
                    r" PROC
    ",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"5"),
                (TokenKind::Plain, r" * "),
                (TokenKind::Number, r"8"),
                (
                    TokenKind::Plain,
                    r"  

    ",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" -"),
                (TokenKind::Number, r"11"),
                (
                    TokenKind::Plain,
                    r"           
    ",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" GetStdHandle

    ",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r"  rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax       
    ",
                ),
                (TokenKind::Keyword, r"lea"),
                (TokenKind::Plain, r"  rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" msg    
    ",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r"  r8"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" LENGTHOF msg - "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"lea"),
                (TokenKind::Plain, r"  r9"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" bytesWritten  
    ",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r"  QWORD PTR "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rsp + "),
                (TokenKind::Number, r"4"),
                (TokenKind::Plain, r" * SIZEOF QWORD"),
                (TokenKind::Punctuation, r"],"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" WriteConsoleA

    ",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"      
    ",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" ExitProcess
",
                ),
                (TokenKind::Keyword, r"main"),
                (
                    TokenKind::Plain,
                    r" ENDP
",
                ),
                (TokenKind::Keyword, r"END"),
            ],
        }],
    };
