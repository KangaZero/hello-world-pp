//! Hello-world snippet for `ProgrammingLanguageName::AssemblerPdp10Midas`.
//!
//! Imported from leachim6/hello-world (`a/Assembler pdp10 midas.mid` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20pdp10%20midas.mid>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerPdp10Midas => "Assembler pdp10 midas",
    versions: [AssemblerPdp10Midas => "Assembler pdp10 midas"],
    extensions: [Mid => "mid"],
}

const ASSEMBLER_PDP10_MIDAS: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerPdp10Midas,
        extensions: &[FileExtension::Mid],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerPdp10Midas,
            syntax: ASSEMBLER_PDP10_MIDAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_PDP10_MIDAS,
                    [
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Keyword, r"TITLE"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" ",
                                output_for_hello_world!(),
                                r"

"
                            ),
                        ),
                        (TokenKind::Keyword, r"A"),
                        (TokenKind::Plain, r"=="),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"B"),
                        (TokenKind::Plain, r"=="),
                        (TokenKind::Number, r"2"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"TYO"),
                        (TokenKind::Plain, r"=="),
                        (TokenKind::Number, r"1"),
                        (
                            TokenKind::Plain,
                            r"

START",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".OPEN"),
                        (TokenKind::Plain, r" TYO"),
                        (TokenKind::Punctuation, r",["),
                        (TokenKind::Macro, r".UAO"),
                        (TokenKind::Punctuation, r",,"),
                        (TokenKind::String, r"'TTY]"),
                        (
                            TokenKind::Plain,
                            r"
	 ",
                        ),
                        (TokenKind::Macro, r".LOSE"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"MOVE"),
                        (TokenKind::Plain, r" A"),
                        (TokenKind::Punctuation, r",["),
                        (TokenKind::Number, r"440700"),
                        (TokenKind::Punctuation, r",,"),
                        (TokenKind::Plain, r"HELLO"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"
LOOP",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"	ILDB B"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"A
	",
                        ),
                        (TokenKind::Keyword, r"JUMPE"),
                        (TokenKind::Plain, r" B"),
                        (TokenKind::Punctuation, r",["),
                        (TokenKind::Macro, r".LOGOUT"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"1"),
                        (TokenKind::Punctuation, r",]"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Macro, r".IOT"),
                        (TokenKind::Plain, r" TYO"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"B
	",
                        ),
                        (TokenKind::Keyword, r"JRST"),
                        (
                            TokenKind::Plain,
                            r" LOOP

HELLO",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Keyword, r"ASCIZ"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" /",
                                output_for_hello_world!(),
                                r"/

	"
                            ),
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" START"),
                    ],
                )
            },
        }],
    };
