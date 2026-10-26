//! Hello-world snippet for `ProgrammingLanguageName::AssemblerLc3`.
//!
//! Imported from leachim6/hello-world (`a/Assembler lc3.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20lc3.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerLc3 => "Assembler lc3",
    versions: [AssemblerLc3 => "Assembler lc3"],
    extensions: [],
}

const ASSEMBLER_LC3: &str = concat!(
    r#"			.ORIG x3000

			LEA R0, HELLOWORLD
			PUTS
			HALT
	
HELLOWORLD	.STRINGZ ""#,
    output_for_hello_world!(),
    r#"\n"

			.END"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerLc3,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerLc3,
            syntax: ASSEMBLER_LC3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_LC3,
                    [
                        (TokenKind::Plain, r"			"),
                        (TokenKind::Macro, r".ORIG"),
                        (
                            TokenKind::Plain,
                            r" x3000

			",
                        ),
                        (TokenKind::Keyword, r"LEA"),
                        (TokenKind::Plain, r" R0"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" HELLOWORLD
			",
                        ),
                        (TokenKind::Keyword, r"PUTS"),
                        (
                            TokenKind::Plain,
                            r"
			",
                        ),
                        (TokenKind::Keyword, r"HALT"),
                        (
                            TokenKind::Plain,
                            r"
	
",
                        ),
                        (TokenKind::Keyword, r"HELLOWORLD"),
                        (TokenKind::Plain, r"	"),
                        (TokenKind::Macro, r".STRINGZ"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"

			",
                        ),
                        (TokenKind::Macro, r".END"),
                    ],
                )
            },
        }],
    };
