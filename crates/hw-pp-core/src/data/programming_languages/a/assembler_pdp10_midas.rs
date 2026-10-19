//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp10Midas`.
//!
//! Imported from leachim6/hello-world (`a/Assembler pdp10 midas.mid` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerPdp10Midas => "Assembler pdp10 midas",
    versions: [AssemblerPdp10Midas => "Assembler pdp10 midas"],
    extensions: [Mid => "mid"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp10Midas,
        extensions: &[FileExtension::Mid],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp10Midas,
            syntax: concat!(
                r"	TITLE ",
                output_for_hello_world!(),
                r"

A==1
B==2
TYO==1

START:	.OPEN TYO,[.UAO,,'TTY]
	 .LOSE
	MOVE A,[440700,,HELLO]
LOOP:	ILDB B,A
	JUMPE B,[.LOGOUT 1,]
	.IOT TYO,B
	JRST LOOP

HELLO:	ASCIZ /",
                output_for_hello_world!(),
                r"/

	END START"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"	TITLE ",
                    output_for_hello_world!(),
                    r"

A==1
B==2
TYO==1

START:	.OPEN TYO,[.UAO,,'TTY]
	 .LOSE
	MOVE A,[440700,,HELLO]
LOOP:	ILDB B,A
	JUMPE B,[.LOGOUT 1,]
	.IOT TYO,B
	JRST LOOP

HELLO:	ASCIZ /",
                    output_for_hello_world!(),
                    r"/

	END START"
                ),
            )],
        }],
    };
