//! Hello-world snippet for `ProgrammingLanguageName::Reason`.
//!
//! Imported from leachim6/hello-world (`r/Reason.re` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Reason.re>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Ml, OCaml},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Reason => "Reason",
    versions: [Reason => "Reason"],
    extensions: [Re => "re"],
}

const REASON: &str = concat!(r#"print_string ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Reason,
        extensions: &[FileExtension::Re],
        categories: &[Compiled],
        families: &[Ml, OCaml],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Reason,
            syntax: REASON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REASON,
                    [
                        (TokenKind::Function, r"print_string"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
