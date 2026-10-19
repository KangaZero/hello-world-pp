//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Zxspectrum`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 zxspectrum.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Z80%20zxspectrum.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ80Zxspectrum => "Assembler Z80 zxspectrum",
    versions: [AssemblerZ80Zxspectrum => "Assembler Z80 zxspectrum"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Zxspectrum,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Zxspectrum,
            syntax: concat!(
                r#"org $6000
	ld bc, STRING
	ld de, SCR

LOOP
	ld a, (bc)
	cp 0
	jr z, EXIT
	rst $10
	inc bc
	inc de
	jr LOOP

EXIT
	ret
	
SCR equ 16384

STRING
	defb ""#,
                output_for_hello_world!(),
                r#""
	defb 13, 0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"org $6000
	ld bc, STRING
	ld de, SCR

LOOP
	ld a, (bc)
	cp 0
	jr z, EXIT
	rst $10
	inc bc
	inc de
	jr LOOP

EXIT
	ret
	
SCR equ 16384

STRING
	defb ""#,
                    output_for_hello_world!(),
                    r#""
	defb 13, 0"#
                ),
            )],
        }],
    };
