//! Hello-world snippet for `ProgrammingLanguageName::Mips`.
//!
//! Imported from leachim6/hello-world (`m/Mips.s` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mips.s>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Assembly],
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
            tokenized_syntax: &[
                (TokenKind::Macro, r".data"),
                (
                    TokenKind::Plain,
                    r"
hello_world",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Macro, r".asciiz"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Macro, r".text"),
                (
                    TokenKind::Plain,
                    r"
main",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"li"),
                (TokenKind::Plain, r" $v0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"4"),
                (TokenKind::Plain, r"            "),
                (TokenKind::Comment, r"# Load syscommand print_string"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"la"),
                (TokenKind::Plain, r" $a0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" hello_world  "),
                (
                    TokenKind::Comment,
                    r"# Load hello_world string into register $a0",
                ),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"syscall"),
                (TokenKind::Plain, r"              "),
                (TokenKind::Comment, r"# Print the string"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"jr"),
                (TokenKind::Plain, r" $ra               "),
                (TokenKind::Comment, r"# Return"),
            ],
        }],
    };
