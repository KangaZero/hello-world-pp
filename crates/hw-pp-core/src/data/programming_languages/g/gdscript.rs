//! Hello-world snippet for `ProgrammingLanguageName::Gdscript`.
//!
//! Imported from leachim6/hello-world (`g/GDScript.gd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GDScript.gd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gdscript => "GDScript",
    versions: [Gdscript => "GDScript"],
    extensions: [Gd => "gd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gdscript,
        extensions: &[FileExtension::Gd],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gdscript,
            syntax: concat!(
                r#"extends Node2D

func _ready():
	print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"extends"),
                (
                    TokenKind::Plain,
                    r" Node2D

",
                ),
                (TokenKind::Keyword, r"func"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"_ready"),
                (TokenKind::Punctuation, r"():"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Function, r"print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
