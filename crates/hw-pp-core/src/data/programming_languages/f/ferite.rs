//! Hello-world snippet for `ProgrammingLanguageName::Ferite`.
//!
//! Imported from leachim6/hello-world (`f/Ferite.fe` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Ferite.fe>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ferite => "Ferite",
    versions: [Ferite => "Ferite"],
    extensions: [Fe => "fe"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ferite,
        extensions: &[FileExtension::Fe],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ferite,
            syntax: concat!(
                r#"uses "console";
Console.println( ""#,
                output_for_hello_world!(),
                r#"" );"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"uses "),
                (TokenKind::String, r#""console""#),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
Console",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"println"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };
