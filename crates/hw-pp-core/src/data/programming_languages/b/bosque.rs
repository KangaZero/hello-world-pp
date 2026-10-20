//! Hello-world snippet for `ProgrammingLanguageName::Bosque`.
//!
//! Imported from leachim6/hello-world (`b/Bosque.bsq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bosque.bsq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bosque => "Bosque",
    versions: [Bosque => "Bosque"],
    extensions: [Bsq => "bsq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bosque,
        extensions: &[FileExtension::Bsq],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bosque,
            syntax: concat!(
                r#"namespace NSMain;

entrypoint function main(): String {  
    return ""#,
                output_for_hello_world!(),
                r#"";
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"namespace"),
                (TokenKind::Plain, r" NSMain"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

entrypoint ",
                ),
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"():"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"String"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
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
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
