//! Hello-world snippet for `ProgrammingLanguageName::Cobra`.
//!
//! Imported from leachim6/hello-world (`c/Cobra.cobra` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cobra.cobra>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cobra => "Cobra",
    versions: [Cobra => "Cobra"],
    extensions: [Cobra => "cobra"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobra,
        extensions: &[FileExtension::Cobra],
        categories: &[Compiled],
        families: &[DotNet],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobra,
            syntax: concat!(
                r"class Hello

    def main
        print '",
                output_for_hello_world!(),
                r"'"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (
                    TokenKind::Plain,
                    r" Hello

    ",
                ),
                (TokenKind::Keyword, r"def"),
                (
                    TokenKind::Plain,
                    r" main
        ",
                ),
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
            ],
        }],
    };
