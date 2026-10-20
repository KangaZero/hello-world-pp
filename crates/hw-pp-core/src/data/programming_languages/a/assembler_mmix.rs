//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMmix`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MMIX.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MMIX.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerMmix => "Assembler MMIX",
    versions: [AssemblerMmix => "Assembler MMIX"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMmix,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMmix,
            syntax: concat!(
                r#"t	IS	$255
	LOC	Data_Segment
	GREG	@
String	BYTE	""#,
                output_for_hello_world!(),
                r#"",#a,0

	LOC	#100
Main	LDA	t,String
	TRAP	0,Fputs,StdOut
	TRAP	0,Halt,0"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"t	IS	$255
	LOC	Data_Segment
	GREG	@
String	BYTE	""#,
                    output_for_hello_world!(),
                    r#"",#a,0

	LOC	#100
Main	LDA	t,String
	TRAP	0,Fputs,StdOut
	TRAP	0,Halt,0"#
                ),
            )],
        }],
    };
