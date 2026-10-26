//! Hello-world snippet for `ProgrammingLanguageName::AssemblerSparc`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(SPARC).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28SPARC%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(SPARC)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerSparc => "Assembler (SPARC)",
    versions: [AssemblerSparc => "Assembler (SPARC)"],
    extensions: [],
}

const ASSEMBLER_SPARC: &str = concat!(
    r"! ",
    output_for_hello_world!(),
    r#" in SPARC Assembly Language

	.section			".data1"
	.align		4
.L16:
	.ascii   ""#,
    output_for_hello_world!(),
    r#"\n\0"

	.section  ".text"
	.global  main
main:
	save  %sp,-96,%sp
	set  .L16,%o0
	call  printf,1
	nop
	restore"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerSparc,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerSparc,
            syntax: ASSEMBLER_SPARC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_SPARC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"! ",
                            output_for_hello_world!(),
                            r#" in SPARC Assembly Language

	.section			".data1"
	.align		4
.L16:
	.ascii   ""#,
                            output_for_hello_world!(),
                            r#"\n\0"

	.section  ".text"
	.global  main
main:
	save  %sp,-96,%sp
	set  .L16,%o0
	call  printf,1
	nop
	restore"#
                        ),
                    )],
                )
            },
        }],
    };
