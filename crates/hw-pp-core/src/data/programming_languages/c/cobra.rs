//! Hello-world snippet for `ProgrammingLanguageName::Cobra`.
//!
//! Imported from leachim6/hello-world (`c/Cobra.cobra` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cobra.cobra>
//! - <https://www.wikidata.org/wiki/Q608565> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cobra => "Cobra",
    versions: [Cobra => "Cobra"],
    extensions: [Cobra => "cobra"],
}

const COBRA: &str = concat!(
    r"class Hello

    def main
        print '",
    output_for_hello_world!(),
    r"'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobra,
        extensions: &[FileExtension::Cobra],
        categories: &[Compiled],
        families: &[DotNet],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobra,
            syntax: COBRA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COBRA,
                    [
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
                )
            },
        }],
    };
