//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp11Palx`.
//!
//! Imported from leachim6/hello-world (`a/Assembler pdp11 palx.palx` at `a152253`),
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
    language: AssemblerPdp11Palx => "Assembler pdp11 palx",
    versions: [AssemblerPdp11Palx => "Assembler pdp11 palx"],
    extensions: [Palx => "palx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp11Palx,
        extensions: &[FileExtension::Palx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp11Palx,
            syntax: concat!(
                r"	.TITLE ",
                output_for_hello_world!(),
                r"

R0=%0
R1=%1
TPS==177564
TPB==177566

	.=400

START:	MOV #1,@#TPS
	MOV #HELLO,R0
LOOP:	TSTB @#TPS
	BPL LOOP
	MOVB (R0)+,R1
	BEQ STOP
	MOVB R1,@#TPB
	BR LOOP

STOP:	HALT

HELLO:	.ASCIZ /",
                output_for_hello_world!(),
                r"/

	.END START"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"	.TITLE ",
                    output_for_hello_world!(),
                    r"

R0=%0
R1=%1
TPS==177564
TPB==177566

	.=400

START:	MOV #1,@#TPS
	MOV #HELLO,R0
LOOP:	TSTB @#TPS
	BPL LOOP
	MOVB (R0)+,R1
	BEQ STOP
	MOVB R1,@#TPB
	BR LOOP

STOP:	HALT

HELLO:	.ASCIZ /",
                    output_for_hello_world!(),
                    r"/

	.END START"
                ),
            )],
        }],
    };
