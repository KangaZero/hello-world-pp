//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmLinux`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Linux.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20Linux.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerNasmLinux => "Assembler NASM Linux",
    versions: [AssemblerNasmLinux => "Assembler NASM Linux"],
    extensions: [],
}

const ASSEMBLER_NASM_LINUX: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmLinux,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmLinux,
            syntax: ASSEMBLER_NASM_LINUX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_NASM_LINUX,
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

_start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"			

	",
                        ),
                        (TokenKind::Keyword, r"xor"),
                        (TokenKind::Plain, r"	ebx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"ebx 
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	ecx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"msg	
	",
                        ),
                        (TokenKind::Keyword, r"lea"),
                        (TokenKind::Plain, r"	edx"),
                        (TokenKind::Punctuation, r",["),
                        (TokenKind::Plain, r"ebx+len"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"lea"),
                        (TokenKind::Plain, r"	eax"),
                        (TokenKind::Punctuation, r",["),
                        (TokenKind::Plain, r"ebx+"),
                        (TokenKind::Number, r"4"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"inc"),
                        (
                            TokenKind::Plain,
                            r"	ebx
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
                        (TokenKind::Keyword, r"xor"),
                        (TokenKind::Plain, r"	eax"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" eax
	",
                        ),
                        (TokenKind::Keyword, r"inc"),
                        (
                            TokenKind::Plain,
                            r"	eax	
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
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".rodata"),
                        (
                            TokenKind::Plain,
                            r"

msg	",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
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
