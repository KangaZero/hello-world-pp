//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmLinux`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Linux.asm` at `a152253`),
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
    language: AssemblerNasmLinux => "Assembler NASM Linux",
    versions: [AssemblerNasmLinux => "Assembler NASM Linux"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmLinux,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmLinux,
            syntax: concat!(
                r"section	.text
    global _start		

_start:			

	xor	ebx,ebx 
	mov	ecx,msg	
	lea	edx,[ebx+len]
	lea	eax,[ebx+4]
	inc	ebx
	int	0x80	

	xor	eax, eax
	inc	eax	
	int	0x80	

section	.rodata

msg	db	'",
                output_for_hello_world!(),
                r"',0xa
len	equ	$ - msg"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"section	.text
    global _start		

_start:			

	xor	ebx,ebx 
	mov	ecx,msg	
	lea	edx,[ebx+len]
	lea	eax,[ebx+4]
	inc	ebx
	int	0x80	

	xor	eax, eax
	inc	eax	
	int	0x80	

section	.rodata

msg	db	'",
                    output_for_hello_world!(),
                    r"',0xa
len	equ	$ - msg"
                ),
            )],
        }],
    };
