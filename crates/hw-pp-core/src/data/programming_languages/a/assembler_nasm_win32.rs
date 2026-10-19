//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmWin32`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Win32.asm` at `a152253`),
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
    language: AssemblerNasmWin32 => "Assembler NASM Win32",
    versions: [AssemblerNasmWin32 => "Assembler NASM Win32"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmWin32,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmWin32,
            syntax: concat!(
                r#"BITS	32
global	_main
extern	_printf
section	.text
	push	msg
	call	_printf
	add		esp,4
	ret
section	.data
	msg:	db	""#,
                output_for_hello_world!(),
                r#"",0Ah,0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"BITS	32
global	_main
extern	_printf
section	.text
	push	msg
	call	_printf
	add		esp,4
	ret
section	.data
	msg:	db	""#,
                    output_for_hello_world!(),
                    r#"",0Ah,0"#
                ),
            )],
        }],
    };
