//! Hello-world snippet for `ProgrammingLanguageName::Erg`.
//!
//! Imported from leachim6/hello-world (`e/Erg.er` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Erg.er>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Erg => "Erg",
    versions: [Erg => "Erg"],
    extensions: [Er => "er"],
}

const ERG: &str = concat!(r#"print! ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erg,
        extensions: &[FileExtension::Er],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Erg,
            syntax: ERG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ERG,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r"! "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
