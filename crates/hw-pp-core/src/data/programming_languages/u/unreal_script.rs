//! Hello-world snippet for `ProgrammingLanguageName::UnrealScript`.
//!
//! Imported from leachim6/hello-world (`u/Unreal Script.uc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Unreal%20Script.uc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: UnrealScript => "Unreal Script",
    versions: [UnrealScript => "Unreal Script"],
    extensions: [Uc => "uc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::UnrealScript,
        extensions: &[FileExtension::Uc],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::UnrealScript,
            syntax: concat!(
                r#"class HelloWorld extends Mutator;

function PostBeginPlay()
{
  Super.PostBeginPlay();
  Log(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" HelloWorld "),
                (TokenKind::Keyword, r"extends"),
                (TokenKind::Plain, r" Mutator"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"PostBeginPlay"),
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
  Super",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"PostBeginPlay"),
                (TokenKind::Punctuation, r"();"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Function, r"Log"),
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
            ],
        }],
    };
