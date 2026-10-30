//! Hello-world snippet for `ProgrammingLanguageName::Elixir`.
//!
//! Sources:
//! - <https://elixir.hexdocs.pm/introduction.html>
//! - <https://elixir.hexdocs.pm/IO.html>
//! - <https://www.wikidata.org/wiki/Q5362035> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Elixir => "Elixir",
    versions: [Elixir1 => "Elixir 1"],
    extensions: [Ex => "ex", Exs => "exs"],
}

const ELIXIR1: &str = concat!(r#"IO.puts(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elixir,
        extensions: &[FileExtension::Ex, FileExtension::Exs],
        categories: &[Compiled, Scripting],
        families: &[],
        paradigms: &[Functional, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elixir1,
            syntax: ELIXIR1,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELIXIR1,
                    [
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
                )
            },
        }],
    };
