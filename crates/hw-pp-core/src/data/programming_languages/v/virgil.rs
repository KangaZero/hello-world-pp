//! Hello-world snippet for `ProgrammingLanguageName::Virgil`.
//!
//! Imported from leachim6/hello-world (`v/Virgil.v3` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Virgil.v3>
//! - <https://github.com/titzer/virgil> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Virgil => "Virgil",
    versions: [Virgil => "Virgil"],
    extensions: [V3 => "v3"],
}

const VIRGIL: &str = concat!(
    r#" def main() {
        System.puts(""#,
    output_for_hello_world!(),
    r#"\n");
 }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Virgil,
        extensions: &[FileExtension::V3],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Virgil,
            syntax: VIRGIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VIRGIL,
                    [
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
        System",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"puts"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
