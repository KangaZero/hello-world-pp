//! Hello-world snippet for `ProgrammingLanguageName::AssemblerBootsect`.
//!
//! Imported from leachim6/hello-world (`a/Assembler Bootsect.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20Bootsect.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerBootsect => "Assembler Bootsect",
    versions: [AssemblerBootsect => "Assembler Bootsect"],
    extensions: [],
}

const ASSEMBLER_BOOTSECT: &str = concat!(
    r#"[ORG	0x7C00]

	mov	si,HelloWorld
	call	print_string

	jmp	$
	
	HelloWorld	db	""#,
    output_for_hello_world!(),
    r#"",0xD,0xA,0
	
print_string:
	mov	ah,0xE
.repeat:
	lodsb
	cmp	al,0
	je	.done
	int	0x10
	jmp	.repeat
.done:
	ret
	
	times	510-($-$$)	db	0
	dw	0xAA55"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerBootsect,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerBootsect,
            syntax: ASSEMBLER_BOOTSECT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_BOOTSECT,
                    [
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"ORG	"),
                        (TokenKind::Number, r"0x7C00"),
                        (TokenKind::Punctuation, r"]"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	si"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"HelloWorld
	",
                        ),
                        (TokenKind::Keyword, r"call"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Function, r"print_string"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (
                            TokenKind::Plain,
                            r"	$
	
	HelloWorld	",
                        ),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0xD"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0xA"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
	
print_string",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"mov"),
                        (TokenKind::Plain, r"	ah"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0xE"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".repeat"),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"lodsb"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"cmp"),
                        (TokenKind::Plain, r"	al"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"je"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".done"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"int"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"0x10"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"jmp"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".repeat"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".done"),
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
	
	",
                        ),
                        (TokenKind::Keyword, r"times"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"510"),
                        (TokenKind::Plain, r"-"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"$-$$"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Keyword, r"db"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"dw"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Number, r"0xAA55"),
                    ],
                )
            },
        }],
    };
