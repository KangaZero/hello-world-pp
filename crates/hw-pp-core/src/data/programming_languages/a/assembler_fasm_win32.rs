//! Hello-world snippet for `ProgrammingLanguageName::AssemblerFasmWin32`.
//!
//! Imported from leachim6/hello-world (`a/Assembler FASM Win32.asm` at `a152253`),
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
    language: AssemblerFasmWin32 => "Assembler FASM Win32",
    versions: [AssemblerFasmWin32 => "Assembler FASM Win32"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerFasmWin32,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerFasmWin32,
            syntax: concat!(
                r#"format PE CONSOLE
entry start
include 'win32a.inc'
section '.data' data readable writable
	helo db ""#,
                output_for_hello_world!(),
                r#"", 0

section '.text' code readable executable
start:	xor ecx, ecx
	push helo
	call [printf]
	add esp, 4
	
	
	push ecx
	call [ExitProcess]



section '.idata'  import data readable
library kernel 	, 'kernel32.dll',\
	msvcrt , 'msvcrt.dll '

import kernel,\
	ExitProcess, 'ExitProcess'
import msvcrt,\
	printf, 'printf'"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"format PE CONSOLE
entry start
include 'win32a.inc'
section '.data' data readable writable
	helo db ""#,
                    output_for_hello_world!(),
                    r#"", 0

section '.text' code readable executable
start:	xor ecx, ecx
	push helo
	call [printf]
	add esp, 4
	
	
	push ecx
	call [ExitProcess]



section '.idata'  import data readable
library kernel 	, 'kernel32.dll',\
	msvcrt , 'msvcrt.dll '

import kernel,\
	ExitProcess, 'ExitProcess'
import msvcrt,\
	printf, 'printf'"#
                ),
            )],
        }],
    };
