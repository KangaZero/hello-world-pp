//! Hello-world snippet for `ProgrammingLanguageName::Concurnas`.
//!
//! Imported from leachim6/hello-world (`c/Concurnas.conc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Concurnas.conc>
//! - <https://github.com/Concurnas/Concurnas> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Concurnas => "Concurnas",
    versions: [Concurnas => "Concurnas"],
    extensions: [Conc => "conc"],
}

const CONCURNAS: &str = concat!(
    r#"System.out.println(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Concurnas,
        extensions: &[FileExtension::Conc],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Concurnas,
            syntax: CONCURNAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CONCURNAS,
                    [
                        (TokenKind::Plain, r"System"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"out"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"println"),
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
