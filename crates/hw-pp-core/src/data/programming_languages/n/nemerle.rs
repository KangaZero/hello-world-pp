//! Hello-world snippet for `ProgrammingLanguageName::Nemerle`.
//!
//! Imported from leachim6/hello-world (`n/Nemerle.n` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nemerle.n>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Nemerle => "Nemerle",
    versions: [Nemerle => "Nemerle"],
    extensions: [N => "n"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nemerle,
        extensions: &[FileExtension::N],
        categories: &[Compiled],
        families: &[DotNet],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nemerle,
            syntax: concat!(
                r#"class Hello
{
    static Main () : void
    {
        System.Console.WriteLine (""#,
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (
                    TokenKind::Plain,
                    r" Hello
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Main"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"void"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
        System",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Console"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"WriteLine"),
                (TokenKind::Plain, r" "),
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
