//! Hello-world snippet for `ProgrammingLanguageName::Zinc`.
//!
//! Imported from leachim6/hello-world (`z/Zinc.zn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zinc.zn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zinc => "Zinc",
    versions: [Zinc => "Zinc"],
    extensions: [Zn => "zn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zinc,
        extensions: &[FileExtension::Zn],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zinc,
            syntax: concat!(
                r#"//! zinc
library HelloWorld {
    private function onInit() {
        DisplayTimedTextToPlayer(GetLocalPlayer(), 0, 0, 0, ""#,
                output_for_hello_world!(),
                r#"");
    }
}
//! endzinc"#
            ),
            tokenized_syntax: &[
                (TokenKind::Comment, r"//! zinc"),
                (
                    TokenKind::Plain,
                    r"
library HelloWorld ",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"private"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"onInit"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Function, r"DisplayTimedTextToPlayer"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"GetLocalPlayer"),
                (TokenKind::Punctuation, r"(),"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
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
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Comment, r"//! endzinc"),
            ],
        }],
    };
