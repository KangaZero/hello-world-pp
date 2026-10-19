//! Hello-world snippet for `ProgrammingLanguageName::Assembler6502appleii`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6502appleII.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"STROUT 	EQU	$DB3A
	LDY	#>HELLO
	LDA	#<HELLO
	JMP	STROUT

HELLO	ASC	""#,
                    output_for_hello_world!(),
                    r#"",00"#
                ),
            )],
        }],
    };
