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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Assembler6502 => "Assembler 6502",
    versions: [Assembler6502 => "Assembler 6502"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assembler6502,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assembler6502,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
