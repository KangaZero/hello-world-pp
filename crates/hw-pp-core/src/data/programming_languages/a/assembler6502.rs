//! Hello-world snippet for `ProgrammingLanguageName::Assembler6502`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 6502.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%206502.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler6502 => "Assembler 6502",
    versions: [Assembler6502 => "Assembler 6502"],
    extensions: [],
}

const ASSEMBLER6502: &str = concat!(
    r#"a_cr	= $0d		
bsout	= $ffd2		
 
	.code
 
	ldx #0		
printnext:
	lda text,x	
	beq done	
	jsr bsout	
	inx		
	bne printnext	
done:
	rts		
	.rodata
 
text:
	.byte	""#,
    output_for_hello_world!(),
    r#"", a_cr, 0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6502,
            syntax: ASSEMBLER6502,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER6502,
                    [
                        (TokenKind::Keyword, r"a_cr"),
                        (
                            TokenKind::Plain,
                            r"	= $0d		
",
                        ),
                        (TokenKind::Keyword, r"bsout"),
                        (
                            TokenKind::Plain,
                            r"	= $ffd2		
 
	",
                        ),
                        (TokenKind::Macro, r".code"),
                        (
                            TokenKind::Plain,
                            r"
 
	",
                        ),
                        (TokenKind::Keyword, r"ldx"),
                        (TokenKind::Plain, r" #"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"		
printnext",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	lda ",
                        ),
                        (TokenKind::Keyword, r"text"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"x	
	",
                        ),
                        (TokenKind::Keyword, r"beq"),
                        (
                            TokenKind::Plain,
                            r" done	
	",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (
                            TokenKind::Plain,
                            r" bsout	
	",
                        ),
                        (TokenKind::Keyword, r"inx"),
                        (
                            TokenKind::Plain,
                            r"		
	",
                        ),
                        (TokenKind::Keyword, r"bne"),
                        (
                            TokenKind::Plain,
                            r" printnext	
done",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"rts"),
                        (
                            TokenKind::Plain,
                            r"		
	",
                        ),
                        (TokenKind::Macro, r".rodata"),
                        (
                            TokenKind::Plain,
                            r"
 
text",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Macro, r".byte"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" a_cr"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                    ],
                )
            },
        }],
    };
