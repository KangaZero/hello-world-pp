//! Hello-world snippet for `ProgrammingLanguageName::Gocaml`.
//!
//! Imported from leachim6/hello-world (`g/Gocaml.ml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gocaml.ml>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gocaml => "Gocaml",
    versions: [Gocaml => "Gocaml"],
    extensions: [],
}

const GOCAML: &str = concat!(r#"println_str ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gocaml,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gocaml,
            syntax: GOCAML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOCAML,
                    [
                        (TokenKind::Plain, r"println_str "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
