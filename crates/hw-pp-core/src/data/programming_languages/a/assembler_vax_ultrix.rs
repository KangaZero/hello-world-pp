//! Hello-world snippet for `ProgrammingLanguageName::AssemblerVaxUltrix`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Vax Ultrix.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Vax%20Ultrix.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerVaxUltrix => "Assembler Vax Ultrix",
    versions: [AssemblerVaxUltrix => "Assembler Vax Ultrix"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerVaxUltrix,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerVaxUltrix,
            syntax: concat!(
                r#".data
hw:
	.ascii ""#,
                output_for_hello_world!(),
                r#"\12"
.text
	.align 1
.globl _main
_main:
	.word 0
	pushl $13
	pushab hw
	pushl $1
	calls $3,_write
	ret"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#".data
hw:
	.ascii ""#,
                    output_for_hello_world!(),
                    r#"\12"
.text
	.align 1
.globl _main
_main:
	.word 0
	pushl $13
	pushab hw
	pushl $1
	calls $3,_write
	ret"#
                ),
            )],
        }],
    };
