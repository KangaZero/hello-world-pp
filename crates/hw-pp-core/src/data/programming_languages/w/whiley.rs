//! Hello-world snippet for `ProgrammingLanguageName::Whiley`.
//!
//! Imported from leachim6/hello-world (`w/Whiley.whiley` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Whiley => "Whiley",
    versions: [Whiley => "Whiley"],
    extensions: [Whiley => "whiley"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Whiley,
        extensions: &[FileExtension::Whiley],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Whiley,
            syntax: concat!(
                r#"import std::ascii
import std::io

method main(ascii::string[] args):
    io::println(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import std::ascii
import std::io

method main(ascii::string[] args):
    io::println(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
