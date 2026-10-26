//! Hello-world snippet for `ProgrammingLanguageName::AssemblerFasmWin32`.
//!
//! Imported from leachim6/hello-world (`a/Assembler FASM Win32.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20FASM%20Win32.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerFasmWin32 => "Assembler FASM Win32",
    versions: [AssemblerFasmWin32 => "Assembler FASM Win32"],
    extensions: [],
}

const ASSEMBLER_FASM_WIN32: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerFasmWin32,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerFasmWin32,
            syntax: ASSEMBLER_FASM_WIN32,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_FASM_WIN32,
                    [
                        (TokenKind::Keyword, r"format"),
                        (
                            TokenKind::Plain,
                            r" PE CONSOLE
",
                        ),
                        (TokenKind::Keyword, r"entry"),
                        (
                            TokenKind::Plain,
                            r" start
",
                        ),
                        (TokenKind::Keyword, r"include"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'win32a.inc'"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'.data'"),
                        (
                            TokenKind::Plain,
                            r" data readable writable
	helo ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'.text'"),
                        (
                            TokenKind::Plain,
                            r" code readable executable
start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"	xor ecx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" ecx
	",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r" helo
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"add"),
                        (TokenKind::Plain, r" esp"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
	
	
	",
                        ),
                        (TokenKind::Keyword, r"push"),
                        (
                            TokenKind::Plain,
                            r" ecx
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"ExitProcess"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"



",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'.idata'"),
                        (
                            TokenKind::Plain,
                            r"  import data readable
",
                        ),
                        (TokenKind::Keyword, r"library"),
                        (TokenKind::Plain, r" kernel 	"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'kernel32.dll'"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"\
	",
                        ),
                        (TokenKind::Keyword, r"msvcrt"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'msvcrt.dll '"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" kernel"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"\
	",
                        ),
                        (TokenKind::Keyword, r"ExitProcess"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'ExitProcess'"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" msvcrt"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"\
	",
                        ),
                        (TokenKind::Keyword, r"printf"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'printf'"),
                    ],
                )
            },
        }],
    };
