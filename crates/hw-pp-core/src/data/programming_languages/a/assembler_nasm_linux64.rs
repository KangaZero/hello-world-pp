//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmLinux64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Linux64.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20Linux64.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerNasmLinux64 => "Assembler NASM Linux64",
    versions: [AssemblerNasmLinux64 => "Assembler NASM Linux64"],
    extensions: [],
}

const ASSEMBLER_NASM_LINUX64: &str = concat!(
    r#"section .rodata
    msg db ""#,
    output_for_hello_world!(),
    r#"", 0xA     
    len equ $- msg           
section	.text
    global _start            

_start:
	mov	eax, 1        
	mov	edi, eax             
	mov	esi, msg           
    mov edx, len
	syscall                  

    mov eax, 60                 
	xor	edi, edi              
	syscall              "#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmLinux64,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmLinux64,
            syntax: ASSEMBLER_NASM_LINUX64,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_NASM_LINUX64,
                    [
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Macro, r".rodata"),
                        (
                            TokenKind::Plain,
                            r"
    msg ",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0xA"),
                        (
                            TokenKind::Plain,
                            r"     
    len ",
                        ),
                        (TokenKind::Keyword, r"equ"),
                        (
                            TokenKind::Plain,
                            r" $- msg           
",
                        ),
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
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	eax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"        
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	edi"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" eax             
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	esi"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" msg           
    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" edx"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" len
	",
                        ),
                        (TokenKind::Keyword, r"syscall"),
                        (
                            TokenKind::Plain,
                            r"                  

    ",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r" eax"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"60"),
                        (
                            TokenKind::Plain,
                            r"                 
	",
                        ),
                        (TokenKind::Keyword, r"xor"),
                        (TokenKind::Plain, r"	edi"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" edi              
	",
                        ),
                        (TokenKind::Keyword, r"syscall"),
                        (TokenKind::Plain, r"              "),
                    ],
                )
            },
        }],
    };
