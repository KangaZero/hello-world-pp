//! Hello-world snippet for `ProgrammingLanguageName::Elixir`.
//!
//! Sources:
//! - <https://elixir.hexdocs.pm/introduction.html>
//! - <https://elixir.hexdocs.pm/IO.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Elixir => "Elixir",
    versions: [Elixir1 => "Elixir 1"],
    extensions: [Ex => "ex", Exs => "exs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elixir,
        extensions: &[FileExtension::Ex, FileExtension::Exs],
        categories: &[Compiled, Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elixir1,
            syntax: concat!(r#"IO.puts(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Type, "IO"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "puts"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    };
