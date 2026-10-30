//! Hello-world snippet for `ProgrammingLanguageName::AssemblerMmix`.
//!
//! Imported from leachim6/hello-world (`a/Assembler MMIX.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20MMIX.asm>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AssemblerMmix => "Assembler MMIX",
    versions: [AssemblerMmix => "Assembler MMIX"],
    extensions: [],
}

const ASSEMBLER_MMIX: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerMmix,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerMmix,
            syntax: ASSEMBLER_MMIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_MMIX,
                    [(
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
                )
            },
        }],
    };
