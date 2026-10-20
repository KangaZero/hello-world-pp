//! Hello-world snippet for `ProgrammingLanguageName::Vyper`.
//!
//! Imported from leachim6/hello-world (PR #1837 `v/Vyper.vy`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1837>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vyper => "Vyper",
    versions: [Vyper => "Vyper"],
    extensions: [Vy => "vy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vyper,
        extensions: &[FileExtension::Vy],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vyper,
            syntax: concat!(
                r#"@external
def hello() -> String[13]:
    return ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Macro, r"@external"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"hello"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" -> "),
                (TokenKind::Function, r"String"),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Number, r"13"),
                (TokenKind::Punctuation, r"]:"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
