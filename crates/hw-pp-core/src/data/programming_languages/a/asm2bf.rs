//! Hello-world snippet for `ProgrammingLanguageName::Asm2bf`.
//!
//! Imported from leachim6/hello-world (`a/Asm2bf.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Asm2bf.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Asm2bf => "Asm2bf",
    versions: [Asm2bf => "Asm2bf"],
    extensions: [],
}

const ASM2BF: &str = concat!(
    r#"STK 2
ORG 0

TXT ""#,
    output_for_hello_world!(),
    r#""
DB_ 0

@LOOP
	RCL R2, R1
	JZ_ R2, 0
	OUT R2
	INC R1
	JMP %LOOPs"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asm2bf,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asm2bf,
            syntax: ASM2BF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASM2BF,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"STK 2
ORG 0

TXT ""#,
                            output_for_hello_world!(),
                            r#""
DB_ 0

@LOOP
	RCL R2, R1
	JZ_ R2, 0
	OUT R2
	INC R1
	JMP %LOOPs"#
                        ),
                    )],
                )
            },
        }],
    };
