//! Hello-world snippet for `ProgrammingLanguageName::Assembler8048Videopac`.
//!
//! Imported from leachim6/hello-world (`a/Assembler 8048 videopac.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%208048%20videopac.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Assembler8048Videopac => "Assembler 8048 videopac",
    versions: [Assembler8048Videopac => "Assembler 8048 videopac"],
    extensions: [],
}

const ASSEMBLER8048_VIDEOPAC: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" for Philips Videopac (Magnavox Odyssey 2)
; by Frog ( https://github.com/petersobolev )
; 25 July 2016

	cpu	8048
	org	400h


	jmp     02C3h
	jmp     0009h
	jmp	timer
	jmp	001Ah
	jmp	start
	jmp	0044h
timer:
	ret


start:

	call	011Ch       

	mov	r0,#010h    
	mov	r3,#40                      
	mov	r4,#100
	mov	r1,#hellostr & 0ffh

	mov	r2,#11 

nextchar:
	mov	a,r1
	movp	a,@a             
	mov	r5,a
	inc	r1       
	mov	r6,#0eh
	call	03EAh
	djnz	r2,nextchar

	call	0127h             


loop:
	jmp 	loop           


hellostr:
	db 	01dh, 012h, 00eh, 00eh, 017h, 00ch, 011h, 017h, 013h, 00eh, 01ah"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler8048Videopac,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler8048Videopac,
            syntax: ASSEMBLER8048_VIDEOPAC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER8048_VIDEOPAC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"; ",
                                output_for_hello_world!(),
                                r" for Philips Videopac (Magnavox Odyssey 2)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"; by Frog ( https://github.com/petersobolev )",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"; 25 July 2016"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"cpu"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"8048"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"org"),
                        (
                            TokenKind::Plain,
                            r"	400h


	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"     02C3h
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"     0009h
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"	timer
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"	001Ah
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"	start
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"	0044h
timer",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"ret"),
                        (
                            TokenKind::Plain,
                            r"


start",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	011Ch       

	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"#010h    
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r3"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"40"),
                        (
                            TokenKind::Plain,
                            r"                      
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r4"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"100"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r1"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"#hellostr & 0ffh

	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r2"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Number, r"11"),
                        (
                            TokenKind::Plain,
                            r" 

nextchar",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	a"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"r1
	",
                        ),
                        (TokenKind::Keyword, r"movp"),
                        (TokenKind::Plain, r"	a"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"@a             
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r5"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"a
	",
                        ),
                        (TokenKind::Keyword, r"inc"),
                        (
                            TokenKind::Plain,
                            r"	r1       
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	r6"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"#0eh
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	03EAh
	",
                        ),
                        (TokenKind::Keyword, r"djnz"),
                        (TokenKind::Plain, r"	r2"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"nextchar

	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (
                            TokenKind::Plain,
                            r"	0127h             


loop",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r" 	loop           


hellostr",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r" 	01dh"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 012h"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 00eh"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 00eh"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 017h"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 00ch"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 011h"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 017h"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 013h"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 00eh"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" 01ah"),
                    ],
                )
            },
        }],
    };
