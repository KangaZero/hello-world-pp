//! Hello-world snippets for `ProgrammingLanguageName::Groovy`.
//!
//! Sources:
//! - <https://groovy-lang.org/semantics.html>
//! - <https://groovy-lang.org/differences.html>
//! - <https://groovy-lang.org/releasenotes/groovy-4.0.html>

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
    language: Groovy => "Groovy",
    versions: [Groovy4 => "Groovy 4"],
    extensions: [Groovy => "groovy", Gvy => "gvy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groovy,
        extensions: &[FileExtension::Groovy, FileExtension::Gvy],
        categories: &[Scripting, Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Groovy4,
            syntax: concat!(r#"println ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "println"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };
