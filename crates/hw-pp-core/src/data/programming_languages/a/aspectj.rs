//! Hello-world snippet for `ProgrammingLanguageName::Aspectj`.
//!
//! Imported from leachim6/hello-world (`a/Aspectj.aj` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Aspectj.aj>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Aspectj => "Aspectj",
    versions: [Aspectj => "Aspectj"],
    extensions: [Aj => "aj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aspectj,
        extensions: &[FileExtension::Aj],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aspectj,
            syntax: concat!(
                r#"System.out.println(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Keyword, r"out"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"println"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };
