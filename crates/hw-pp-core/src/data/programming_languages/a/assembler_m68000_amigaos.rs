//! Hello-world snippet for `ProgrammingLanguageName::AssemblerM68000Amigaos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler m68000 amigaos.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20m68000%20amigaos.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerM68000Amigaos => "Assembler m68000 amigaos",
    versions: [AssemblerM68000Amigaos => "Assembler m68000 amigaos"],
    extensions: [],
}

const ASSEMBLER_M68000_AMIGAOS: &str = concat!(
    r#"

ExecBase     = 4
OpenLibrary  = -552
CloseLibrary = -414
PutStr       = -948

	section "main", code


	move.l	ExecBase, a6
	lea	lib, a1
	moveq	#0, d0
	jsr	(OpenLibrary, a6)
	move.l	d0, a6


	move.l	#msg, d1
	jsr	(PutStr, a6)


	move.l	a6, a1
	move.l	ExecBase, a6
	jsr	(CloseLibrary, a6)

	moveq	#0, d0
	rts

lib	dc.b	"dos.library", 0
msg	dc.b	""#,
    output_for_hello_world!(),
    r#"\n", 0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerM68000Amigaos,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerM68000Amigaos,
            syntax: ASSEMBLER_M68000_AMIGAOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_M68000_AMIGAOS,
                    [
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"ExecBase"),
                        (TokenKind::Plain, r"     = "),
                        (TokenKind::Number, r"4"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"OpenLibrary"),
                        (TokenKind::Plain, r"  = -"),
                        (TokenKind::Number, r"552"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"CloseLibrary"),
                        (TokenKind::Plain, r" = -"),
                        (TokenKind::Number, r"414"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"PutStr"),
                        (TokenKind::Plain, r"       = -"),
                        (TokenKind::Number, r"948"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"section"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""main""#),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" code


	",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"	ExecBase"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" a6
	",
                        ),
                        (TokenKind::Keyword, r"lea"),
                        (TokenKind::Plain, r"	lib"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" a1
	",
                        ),
                        (TokenKind::Keyword, r"moveq"),
                        (TokenKind::Plain, r"	#"),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" d0
	",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"OpenLibrary"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" a6"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"	d0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" a6


	",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"	#msg"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" d1
	",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"PutStr"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" a6"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"


	",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"	a6"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" a1
	",
                        ),
                        (TokenKind::Keyword, r"move"),
                        (TokenKind::Macro, r".l"),
                        (TokenKind::Plain, r"	ExecBase"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" a6
	",
                        ),
                        (TokenKind::Keyword, r"jsr"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"CloseLibrary"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" a6"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

	",
                        ),
                        (TokenKind::Keyword, r"moveq"),
                        (TokenKind::Plain, r"	#"),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" d0
	",
                        ),
                        (TokenKind::Keyword, r"rts"),
                        (
                            TokenKind::Plain,
                            r"

lib	",
                        ),
                        (TokenKind::Keyword, r"dc"),
                        (TokenKind::Macro, r".b"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::String, r#""dos.library""#),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                        (
                            TokenKind::Plain,
                            r"
msg	",
                        ),
                        (TokenKind::Keyword, r"dc"),
                        (TokenKind::Macro, r".b"),
                        (TokenKind::Plain, r"	"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0"),
                    ],
                )
            },
        }],
    };
