//! Hello-world snippet for `ProgrammingLanguageName::Fantom`.
//!
//! Imported from leachim6/hello-world (`f/Fantom.fan` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fantom.fan>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fantom => "Fantom",
    versions: [Fantom => "Fantom"],
    extensions: [Fan => "fan"],
}

const FANTOM: &str = concat!(
    r#"// Hello from Fantom!
class HelloWorld {
  static Void main() {
    echo(""#,
    output_for_hello_world!(),
    r#"")
  }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fantom,
        extensions: &[FileExtension::Fan],
        categories: &[Compiled],
        families: &[DotNet],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fantom,
            syntax: FANTOM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FANTOM,
                    [
                        (TokenKind::Comment, r"// Hello from Fantom!"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" HelloWorld "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"Void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"echo"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
