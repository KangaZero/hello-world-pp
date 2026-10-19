//! Hello-world snippet for `ProgrammingLanguageName::Gocaml`.
//!
//! Imported from leachim6/hello-world (`g/Gocaml.ml` at `a152253`),
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
    language: Gocaml => "Gocaml",
    versions: [Gocaml => "Gocaml"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gocaml,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gocaml,
            syntax: concat!(r#"println_str ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Plain, r"println_str "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
