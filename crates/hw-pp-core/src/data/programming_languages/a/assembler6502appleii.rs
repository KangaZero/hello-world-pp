//! Hello-world snippet for `ProgrammingLanguageName::Assembler6502appleii`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6502appleII.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%206502appleII.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler6502appleii => "Assembler 6502appleII",
    versions: [Assembler6502appleii => "Assembler 6502appleII"],
    extensions: [],
}

const ASSEMBLER6502APPLEII: &str = concat!(
    r#"STROUT 	EQU	$DB3A
	LDY	#>HELLO
	LDA	#<HELLO
	JMP	STROUT

HELLO	ASC	""#,
    output_for_hello_world!(),
    r#"",00"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502appleii,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6502appleii,
            syntax: ASSEMBLER6502APPLEII,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER6502APPLEII,
                    [
                        (TokenKind::Plain, r"STROUT 	"),
                        (TokenKind::Keyword, r"EQU"),
                        (
                            TokenKind::Plain,
                            r"	$DB3A
	",
                        ),
                        (TokenKind::Keyword, r"LDY"),
                        (
                            TokenKind::Plain,
                            r"	#>HELLO
	",
                        ),
                        (TokenKind::Keyword, r"LDA"),
                        (
                            TokenKind::Plain,
                            r"	#<HELLO
	",
                        ),
                        (TokenKind::Keyword, r"JMP"),
                        (
                            TokenKind::Plain,
                            r"	STROUT

",
                        ),
                        (TokenKind::Keyword, r"HELLO"),
                        (TokenKind::Plain, r"	ASC	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"00"),
                    ],
                )
            },
        }],
    };
