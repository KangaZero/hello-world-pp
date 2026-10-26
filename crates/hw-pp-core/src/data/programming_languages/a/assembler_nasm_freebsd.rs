//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmFreebsd`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM FreeBSD.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20FreeBSD.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerNasmFreebsd => "Assembler NASM FreeBSD",
    versions: [AssemblerNasmFreebsd => "Assembler NASM FreeBSD"],
    extensions: [],
}

const ASSEMBLER_NASM_FREEBSD: &str = concat!(
    r#"section	.text
    global _start		

_syscall:		
	int	0x80	
	ret

_start:			

	push	dword len
	push	dword msg
	push	dword 1
	mov	eax,0x4
	call	_syscall	


	add	esp,12	

	push	dword 0	
	mov	eax,0x1	
	call	_syscall	

section	.data

msg	db	""#,
    output_for_hello_world!(),
    r#"",0xa
len	equ	$ - msg"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmFreebsd,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmFreebsd,
            syntax: ASSEMBLER_NASM_FREEBSD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_NASM_FREEBSD,
                    [
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".text"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"global"),
                        (
                            TokenKind::Plain,
                            r" _start		

_syscall",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"		
	",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"0x80"),
                        (
                            TokenKind::Plain,
                            r"	
	",
                        ),
                        (TokenKind::Keyword, r"ret"),
                        (
                            TokenKind::Plain,
                            r"

_start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"			

	push	",
                        ),
                        (TokenKind::Keyword, r"dword"),
                        (
                            TokenKind::Plain,
                            r" len
	push	",
                        ),
                        (TokenKind::Keyword, r"dword"),
                        (
                            TokenKind::Plain,
                            r" msg
	push	",
                        ),
                        (TokenKind::Keyword, r"dword"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	eax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0x4"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	_syscall	


	",
                        ),
                        (TokenKind::Keyword, r"add"),
                        (TokenKind::Plain, r"	esp"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"12"),
                        (
                            TokenKind::Plain,
                            r"	

	push	",
                        ),
                        (TokenKind::Keyword, r"dword"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"	
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	eax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0x1"),
                        (
                            TokenKind::Plain,
                            r"	
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	_syscall	

",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".data"),
                        (
                            TokenKind::Plain,
                            r"

msg	",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0xa"),
                        (
                            TokenKind::Plain,
                            r"
len	",
                        ),
                        (TokenKind::Keyword, r"equ"),
                        (TokenKind::Plain, r"	$ - msg"),
                    ],
                )
            },
        }],
    };
