//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp11Palx`.
//!
//! Imported from leachim6/hello-world (`a/Assembler pdp11 palx.palx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20pdp11%20palx.palx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Assembly],
        families: &[],
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
            tokenized_syntax: &[
                (TokenKind::Plain, r"	"),
                (TokenKind::Macro, r".TITLE"),
                (
                    TokenKind::Plain,
                    concat!(
                        r" ",
                        output_for_hello_world!(),
                        r"

"
                    ),
                ),
                (TokenKind::Keyword, r"R0"),
                (TokenKind::Plain, r"=%"),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"R1"),
                (TokenKind::Plain, r"=%"),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"TPS"),
                (TokenKind::Plain, r"=="),
                (TokenKind::Number, r"177564"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"TPB"),
                (TokenKind::Plain, r"=="),
                (TokenKind::Number, r"177566"),
                (
                    TokenKind::Plain,
                    r"

	",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"="),
                (TokenKind::Number, r"400"),
                (
                    TokenKind::Plain,
                    r"

START",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"	MOV #"),
                (TokenKind::Number, r"1"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"@#TPS
	",
                ),
                (TokenKind::Keyword, r"MOV"),
                (TokenKind::Plain, r" #HELLO"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"R0
LOOP",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"	TSTB @#TPS
	",
                ),
                (TokenKind::Keyword, r"BPL"),
                (
                    TokenKind::Plain,
                    r" LOOP
	",
                ),
                (TokenKind::Keyword, r"MOVB"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"R0"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r"+"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"R1
	",
                ),
                (TokenKind::Keyword, r"BEQ"),
                (
                    TokenKind::Plain,
                    r" STOP
	",
                ),
                (TokenKind::Keyword, r"MOVB"),
                (TokenKind::Plain, r" R1"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"@#TPB
	",
                ),
                (TokenKind::Keyword, r"BR"),
                (
                    TokenKind::Plain,
                    r" LOOP

STOP",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"	HALT

HELLO",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"	"),
                (TokenKind::Macro, r".ASCIZ"),
                (
                    TokenKind::Plain,
                    concat!(
                        r" /",
                        output_for_hello_world!(),
                        r"/

	"
                    ),
                ),
                (TokenKind::Macro, r".END"),
                (TokenKind::Plain, r" START"),
            ],
        }],
    };
