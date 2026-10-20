//! Hello-world snippets for `ProgrammingLanguageName::D`.
//!
//! Sources:
//! - <https://tour.dlang.org/tour/en/welcome/welcome-to-d>
//! - <https://dlang.org/spec/spec.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: D => "D",
    versions: [D2 => "D 2"],
    extensions: [D => "d"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::D,
        extensions: &[FileExtension::D],
        categories: &[Compiled],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::D2,
            syntax: concat!(
                r#"import std.stdio;

void main() {
    writeln(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "import"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "std.stdio"),
                (TokenKind::Plain, ";\n\n"),
                (TokenKind::Type, "void"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Plain, "() {\n    writeln("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ");\n}"),
            ],
        }],
    };
