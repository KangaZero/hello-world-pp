//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmFreebsd`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM FreeBSD.asm` at `a152253`),
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
    language: AssemblerNasmFreebsd => "Assembler NASM FreeBSD",
    versions: [AssemblerNasmFreebsd => "Assembler NASM FreeBSD"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmFreebsd,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmFreebsd,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
