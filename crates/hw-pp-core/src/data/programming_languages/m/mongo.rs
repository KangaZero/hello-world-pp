//! Hello-world snippet for `ProgrammingLanguageName::Mongo`.
//!
//! Imported from leachim6/hello-world (`m/Mongo.sh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mongo => "Mongo",
    versions: [Mongo => "Mongo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mongo,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mongo,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
