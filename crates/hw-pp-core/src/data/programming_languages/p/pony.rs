//! Hello-world snippet for `ProgrammingLanguageName::Pony`.
//!
//! Imported from leachim6/hello-world (`p/Pony.pony` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pony.pony>
//! - <https://www.wikidata.org/wiki/Q51160093> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pony => "Pony",
    versions: [Pony => "Pony"],
    extensions: [Pony => "pony"],
}

const PONY: &str = concat!(
    r#"actor Main
  new create(env: Env) =>
    env.out.print(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pony,
        extensions: &[FileExtension::Pony],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pony,
            syntax: PONY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PONY,
                    [
                        (
                            TokenKind::Plain,
                            r"actor Main
  ",
                        ),
                        (TokenKind::Keyword, r"new"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"create"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"env"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" Env"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r" =>
    env",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"out"),
                        (TokenKind::Punctuation, r"."),
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
