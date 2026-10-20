//! Hello-world snippet for `ProgrammingLanguageName::Lsl`.
//!
//! Imported from leachim6/hello-world (`l/LSL.lsl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LSL.lsl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lsl => "LSL",
    versions: [Lsl => "LSL"],
    extensions: [Lsl => "lsl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lsl,
        extensions: &[FileExtension::Lsl],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lsl,
            syntax: concat!(
                r#"default
{
    state_entry()
    {
        llSay(0, ""#,
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"default"),
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
                (TokenKind::Function, r"state_entry"),
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
                (TokenKind::Function, r"llSay"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
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
