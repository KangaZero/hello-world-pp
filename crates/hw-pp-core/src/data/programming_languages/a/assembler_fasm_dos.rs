//! Hello-world snippet for `ProgrammingLanguageName::AssemblerFasmDos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler FASM DOS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20FASM%20DOS.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerFasmDos => "Assembler FASM DOS",
    versions: [AssemblerFasmDos => "Assembler FASM DOS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerFasmDos,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerFasmDos,
            syntax: concat!(
                r"format MZ

entry .code: start

segment .code
start:
	mov ax, .data
	mov ds, ax
	mov dx, msg 
	mov ah, 9h
	int 21h
	mov ah, 4ch
	int 21h

segment .data
	msg db '",
                output_for_hello_world!(),
                r"', '$'"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"format MZ

entry .code: start

segment .code
start:
	mov ax, .data
	mov ds, ax
	mov dx, msg 
	mov ah, 9h
	int 21h
	mov ah, 4ch
	int 21h

segment .data
	msg db '",
                    output_for_hello_world!(),
                    r"', '$'"
                ),
            )],
        }],
    };
