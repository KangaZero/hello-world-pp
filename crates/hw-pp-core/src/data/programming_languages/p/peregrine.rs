//! Hello-world snippet for `ProgrammingLanguageName::Peregrine`.
//!
//! Imported from leachim6/hello-world (`p/Peregrine.pe` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Peregrine.pe>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Peregrine => "Peregrine",
    versions: [Peregrine => "Peregrine"],
    extensions: [Pe => "pe"],
}

const PEREGRINE: &str = concat!(
    r#"def main():
  printf(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Peregrine,
        extensions: &[FileExtension::Pe],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Peregrine,
            syntax: PEREGRINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PEREGRINE,
                    [
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"():"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"printf"),
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
