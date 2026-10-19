//! Hello-world snippet for `ProgrammingLanguageName::AssemblerTasmDos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler TASM DOS.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20TASM%20DOS.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerTasmDos => "Assembler TASM DOS",
    versions: [AssemblerTasmDos => "Assembler TASM DOS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerTasmDos,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerTasmDos,
            syntax: concat!(
                r#"IDEAL
MODEL SMALL
STACK 100h

DATASEG
msg			db ""#,
                output_for_hello_world!(),
                r#"", 0dh, 0ah, "$"

CODESEG
start:
	mov		ax, @data
	mov		ds, ax
	
	mov		dx, offset msg
	mov		ah, 9
	int		21h
	
	mov		ah, 4ch
	int		21h

end start"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"IDEAL
MODEL SMALL
STACK 100h

DATASEG
msg			db ""#,
                    output_for_hello_world!(),
                    r#"", 0dh, 0ah, "$"

CODESEG
start:
	mov		ax, @data
	mov		ds, ax
	
	mov		dx, offset msg
	mov		ah, 9
	int		21h
	
	mov		ah, 4ch
	int		21h

end start"#
                ),
            )],
        }],
    };
