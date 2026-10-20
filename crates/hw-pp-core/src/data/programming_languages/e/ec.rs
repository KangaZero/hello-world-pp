//! Hello-world snippet for `ProgrammingLanguageName::Ec`.
//!
//! Imported from leachim6/hello-world (`e/Ec.ec` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Ec.ec>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ec => "Ec",
    versions: [Ec => "Ec"],
    extensions: [Ec => "ec"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ec,
        extensions: &[FileExtension::Ec],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ec,
            syntax: concat!(
                r#"class HelloWorldApp : Application
{
   void Main()
   {
      PrintLn(""#,
                output_for_hello_world!(),
                r#"");
   }
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" HelloWorldApp "),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r" Application
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
   ",
                ),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Main"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
   ",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Function, r"PrintLn"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
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
        }],
    };
