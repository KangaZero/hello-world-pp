//! Hello-world snippet for `ProgrammingLanguageName::AssemblerZ80Zxspectrum`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Z80 zxspectrum.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Z80%20zxspectrum.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerZ80Zxspectrum => "Assembler Z80 zxspectrum",
    versions: [AssemblerZ80Zxspectrum => "Assembler Z80 zxspectrum"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerZ80Zxspectrum,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerZ80Zxspectrum,
            syntax: concat!(
                r#"org $6000
	ld bc, STRING
	ld de, SCR

LOOP
	ld a, (bc)
	cp 0
	jr z, EXIT
	rst $10
	inc bc
	inc de
	jr LOOP

EXIT
	ret
	
SCR equ 16384

STRING
	defb ""#,
                output_for_hello_world!(),
                r#""
	defb 13, 0"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"org"),
                (TokenKind::Plain, r" $"),
                (TokenKind::Number, r"6000"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" bc"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"STRING"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" de"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" SCR

",
                ),
                (TokenKind::Keyword, r"LOOP"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"ld"),
                (TokenKind::Plain, r" a"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"bc"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"cp"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"jr"),
                (TokenKind::Plain, r" z"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" EXIT
	",
                ),
                (TokenKind::Keyword, r"rst"),
                (TokenKind::Plain, r" $"),
                (TokenKind::Number, r"10"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"inc"),
                (
                    TokenKind::Plain,
                    r" bc
	",
                ),
                (TokenKind::Keyword, r"inc"),
                (
                    TokenKind::Plain,
                    r" de
	",
                ),
                (TokenKind::Keyword, r"jr"),
                (
                    TokenKind::Plain,
                    r" LOOP

",
                ),
                (TokenKind::Keyword, r"EXIT"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"ret"),
                (
                    TokenKind::Plain,
                    r"
	
SCR ",
                ),
                (TokenKind::Keyword, r"equ"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"16384"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"STRING"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"defb"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"defb"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"13"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
            ],
        }],
    };
