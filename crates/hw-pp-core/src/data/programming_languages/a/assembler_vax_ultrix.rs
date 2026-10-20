//! Hello-world snippet for `ProgrammingLanguageName::AssemblerVaxUltrix`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Vax Ultrix.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Vax%20Ultrix.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerVaxUltrix => "Assembler Vax Ultrix",
    versions: [AssemblerVaxUltrix => "Assembler Vax Ultrix"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerVaxUltrix,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerVaxUltrix,
            syntax: concat!(
                r#".data
hw:
	.ascii ""#,
                output_for_hello_world!(),
                r#"\12"
.text
	.align 1
.globl _main
_main:
	.word 0
	pushl $13
	pushab hw
	pushl $1
	calls $3,_write
	ret"#
            ),
            tokenized_syntax: &[
                (TokenKind::Macro, r".data"),
                (
                    TokenKind::Plain,
                    r"
hw",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Macro, r".ascii"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\12""#),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Macro, r".text"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Macro, r".align"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Macro, r".globl"),
                (
                    TokenKind::Plain,
                    r" _main
_main",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Macro, r".word"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"pushl"),
                (TokenKind::Plain, r" $"),
                (TokenKind::Number, r"13"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"pushab"),
                (
                    TokenKind::Plain,
                    r" hw
	",
                ),
                (TokenKind::Keyword, r"pushl"),
                (TokenKind::Plain, r" $"),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"calls"),
                (TokenKind::Plain, r" $"),
                (TokenKind::Number, r"3"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"_write
	",
                ),
                (TokenKind::Keyword, r"ret"),
            ],
        }],
    };
