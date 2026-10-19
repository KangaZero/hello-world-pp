//! Hello-world snippet for `ProgrammingLanguageName::Mips`.
//!
//! Imported from leachim6/hello-world (`m/Mips.s` at `a152253`),
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
    language: Mips => "Mips",
    versions: [Mips => "Mips"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mips,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mips,
            syntax: concat!(
                r#".data
hello_world: .asciiz ""#,
                output_for_hello_world!(),
                r#""

.text
main:
  li $v0, 4            # Load syscommand print_string
  la $a0, hello_world  # Load hello_world string into register $a0
  syscall              # Print the string
  jr $ra               # Return"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#".data
hello_world: .asciiz ""#,
                    output_for_hello_world!(),
                    r#""

.text
main:
  li $v0, 4            # Load syscommand print_string
  la $a0, hello_world  # Load hello_world string into register $a0
  syscall              # Print the string
  jr $ra               # Return"#
                ),
            )],
        }],
    };
