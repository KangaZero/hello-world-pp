//! Hello-world snippet for `ProgrammingLanguageName::Micropython`.
//!
//! Imported from leachim6/hello-world (`m/MicroPython.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MicroPython.py>
//! - <https://en.wikipedia.org/wiki/Python_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Reflective, Structured,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Micropython => "MicroPython",
    versions: [Micropython => "MicroPython"],
    extensions: [],
}

const MICROPYTHON: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Micropython,
        extensions: &[],
        categories: &[Scripting],
        families: &[Python],
        paradigms: &[
            Imperative,
            Procedural,
            Structured,
            ObjectOriented,
            Functional,
            Reflective,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Micropython,
            syntax: MICROPYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MICROPYTHON,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
