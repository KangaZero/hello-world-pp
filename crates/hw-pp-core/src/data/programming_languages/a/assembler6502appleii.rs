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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler6502appleii => "Assembler 6502appleII",
    versions: [Assembler6502appleii => "Assembler 6502appleII"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502appleii,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6502appleii,
            syntax: concat!(
                r#"STROUT 	EQU	$DB3A
	LDY	#>HELLO
	LDA	#<HELLO
	JMP	STROUT

HELLO	ASC	""#,
                output_for_hello_world!(),
                r#"",00"#
            ),
            tokenized_syntax: &[
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
        }],
    };
