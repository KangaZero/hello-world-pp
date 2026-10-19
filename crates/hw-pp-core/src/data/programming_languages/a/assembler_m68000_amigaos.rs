//! Hello-world snippet for `ProgrammingLanguageName::AssemblerM68000Amigaos`.
//!
//! Imported from leachim6/hello-world (`a/Assembler m68000 amigaos.asm` at `a152253`),
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
    language: AssemblerM68000Amigaos => "Assembler m68000 amigaos",
    versions: [AssemblerM68000Amigaos => "Assembler m68000 amigaos"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerM68000Amigaos,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerM68000Amigaos,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
