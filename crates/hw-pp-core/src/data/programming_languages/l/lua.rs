//! Hello-world snippet for `ProgrammingLanguageName::Lua`.
//!
//! Sources:
//! - <https://www.lua.org/pil/1.html>
//! - <https://www.lua.org/manual/5.4/manual.html#pdf-print>
//! - <https://www.wikidata.org/wiki/Q207316> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, PrototypeBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lua => "Lua",
    versions: [Lua54 => "Lua 5.4"],
    extensions: [Lua => "lua"],
}

const LUA54: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lua,
        extensions: &[FileExtension::Lua],
        categories: &[Scripting],
        families: &[C],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            PrototypeBased,
            Functional,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lua54,
            syntax: LUA54,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LUA54,
                    [
                        (TokenKind::Function, "print"),
                        (TokenKind::Plain, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Plain, ")"),
                    ],
                )
            },
        }],
    };
