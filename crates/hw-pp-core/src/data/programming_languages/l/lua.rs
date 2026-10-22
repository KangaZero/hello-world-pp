//! Hello-world snippet for `ProgrammingLanguageName::Lua`.
//!
//! Sources:
//! - <https://www.lua.org/pil/1.html>
//! - <https://www.lua.org/manual/5.4/manual.html#pdf-print>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lua => "Lua",
    versions: [Lua54 => "Lua 5.4"],
    extensions: [Lua => "lua"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &[FileExtension::Lua],
        categories: &[Scripting],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lua54,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    };
