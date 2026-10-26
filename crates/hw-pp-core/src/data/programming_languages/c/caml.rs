//! Hello-world snippet for `ProgrammingLanguageName::Caml`.
//!
//! Imported from leachim6/hello-world (`c/CAML.ml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CAML.ml>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Ml,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Caml => "CAML",
    versions: [Caml => "CAML"],
    extensions: [],
}

const CAML: &str = concat!(r#"print_endline ""#, output_for_hello_world!(), r#"";;"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Caml,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[Ml],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Caml,
            syntax: CAML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CAML,
                    [
                        (TokenKind::Plain, r"print_endline "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";;"),
                    ],
                )
            },
        }],
    };
