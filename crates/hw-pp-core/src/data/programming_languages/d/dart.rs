//! Hello-world snippet for `ProgrammingLanguageName::Dart`.
//!
//! Sources:
//! - <https://dart.dev/language>
//! - <https://dart.dev/overview>
//! - <https://www.wikidata.org/wiki/Q406009> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{C, JavaScript},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dart => "Dart",
    versions: [Dart3 => "Dart 3"],
    extensions: [Dart => "dart"],
}

const DART3: &str = concat!(
    r"void main() {
    print('",
    output_for_hello_world!(),
    r"');
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &[FileExtension::Dart],
        categories: &[Compiled, Scripting],
        families: &[C, JavaScript],
        paradigms: &[Imperative, ObjectOriented, Functional, Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dart3,
            syntax: DART3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DART3,
                    [
                        (TokenKind::Type, "void"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, "()"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "{"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Function, "print"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("'", output_for_hello_world!(), "'"),
                        ),
                        (TokenKind::Punctuation, ");"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Punctuation, "}"),
                    ],
                )
            },
        }],
    };
