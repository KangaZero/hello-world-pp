//! Hello-world snippet for `ProgrammingLanguageName::Caml`.
//!
//! Imported from leachim6/hello-world (`c/CAML.ml` at `a152253`),
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
    language: Caml => "CAML",
    versions: [Caml => "CAML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Caml,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Caml,
            syntax: concat!(r#"print_endline ""#, output_for_hello_world!(), r#"";;"#),
            tokenized_syntax: &[
                (TokenKind::Plain, r"print_endline "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r";;"),
            ],
        }],
    };
