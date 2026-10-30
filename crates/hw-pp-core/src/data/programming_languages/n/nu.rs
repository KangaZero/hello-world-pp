//! Hello-world snippet for `ProgrammingLanguageName::Nu`.
//!
//! Imported from leachim6/hello-world (`n/Nu.nu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nu.nu>
//! - <https://www.wikidata.org/wiki/Q7067717> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Nu => "Nu",
    versions: [Nu => "Nu"],
    extensions: [Nu => "nu"],
}

const NU: &str = concat!(r#"(puts ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nu,
        extensions: &[FileExtension::Nu],
        categories: &[Scripting],
        families: &[Lisp],
        paradigms: &[
            Imperative,
            Procedural,
            Structured,
            ObjectOriented,
            Functional,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nu,
            syntax: NU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NU,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"puts"),
                        (TokenKind::Plain, r" "),
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
