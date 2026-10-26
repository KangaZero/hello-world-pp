//! Hello-world snippet for `ProgrammingLanguageName::D`.
//!
//! Sources:
//! - <https://tour.dlang.org/tour/en/welcome/welcome-to-d>
//! - <https://dlang.org/spec/spec.html>
//! - <https://www.wikidata.org/wiki/Q319268> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: D => "D",
    versions: [D2 => "D 2"],
    extensions: [D => "d"],
}

const D2: &str = concat!(
    r#"import std.stdio;

void main() {
    writeln(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::D,
        extensions: &[FileExtension::D],
        categories: &[Compiled],
        families: &[C],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::D2,
            syntax: D2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    D2,
                    [
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
                )
            },
        }],
    };
