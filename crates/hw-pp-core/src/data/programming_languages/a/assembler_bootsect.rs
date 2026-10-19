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
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerBootsect => "Assembler Bootsect",
    versions: [AssemblerBootsect => "Assembler Bootsect"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerBootsect,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerBootsect,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
