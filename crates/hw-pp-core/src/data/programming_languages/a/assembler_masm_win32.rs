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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerMasmWin32 => "Assembler MASM Win32",
    versions: [AssemblerMasmWin32 => "Assembler MASM Win32"],
    extensions: [],
}

const ASSEMBLER_MASM_WIN32: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMasmWin32,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMasmWin32,
            syntax: ASSEMBLER_MASM_WIN32,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_MASM_WIN32,
                    [
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
                                r" for Win32 Intel x86 Assembly"
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
                        (TokenKind::Macro, r".386"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".model"),
                        (TokenKind::Plain, r" flat"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"stdcall
",
                        ),
                        (TokenKind::Macro, r".stack"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"4096"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"EXTRN"),
                        (TokenKind::Plain, r" ExitProcess@"),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r" PROC
",
                        ),
                        (TokenKind::Keyword, r"EXTRN"),
                        (TokenKind::Plain, r" GetStdHandle@"),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r" PROC
",
                        ),
                        (TokenKind::Keyword, r"EXTRN"),
                        (TokenKind::Plain, r" WriteConsoleA@"),
                        (TokenKind::Number, r"20"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r" PROC

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
                        (TokenKind::Keyword, r"push"),
                        (TokenKind::Plain, r" -"),
                        (TokenKind::Number, r"11"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r" GetStdHandle@"),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
 
    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"   
    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r" OFFSET bytesWritten
    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (TokenKind::Plain, r" LENGTHOF msg - "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r" OFFSET msg   
    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r" eax     
    ",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r" WriteConsoleA@"),
                        (TokenKind::Number, r"20"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"  
    ",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r" ExitProcess@"),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"main"),
                        (
                            TokenKind::Plain,
                            r" ENDP
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" main"),
                    ],
                )
            },
        }],
    };
