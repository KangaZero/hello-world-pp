//! Hello-world snippet for `ProgrammingLanguageName::Pinecone`.
//!
//! Imported from leachim6/hello-world (`p/Pinecone.pn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pinecone.pn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pinecone => "Pinecone",
    versions: [Pinecone => "Pinecone"],
    extensions: [],
}

const PINECONE: &str = concat!(r"print: ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pinecone,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pinecone,
            syntax: PINECONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PINECONE,
                    [(
                        TokenKind::Plain,
                        concat!(r"print: ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
