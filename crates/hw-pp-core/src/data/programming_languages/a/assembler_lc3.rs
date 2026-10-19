//! Hello-world snippet for `ProgrammingLanguageName::AssemblerLc3`.
//!
//! Imported from leachim6/hello-world (`a/Assembler lc3.asm` at `a152253`),
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
    language: AssemblerLc3 => "Assembler lc3",
    versions: [AssemblerLc3 => "Assembler lc3"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerLc3,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerLc3,
            syntax: concat!(
                r#"			.ORIG x3000

			LEA R0, HELLOWORLD
			PUTS
			HALT
	
HELLOWORLD	.STRINGZ ""#,
                output_for_hello_world!(),
                r#"\n"

			.END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"			.ORIG x3000

			LEA R0, HELLOWORLD
			PUTS
			HALT
	
HELLOWORLD	.STRINGZ ""#,
                    output_for_hello_world!(),
                    r#"\n"

			.END"#
                ),
            )],
        }],
    };
