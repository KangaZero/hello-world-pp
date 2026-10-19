//! Hello-world snippet for `ProgrammingLanguageName::Rotor`.
//!
//! Imported from leachim6/hello-world (`r/Rotor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rotor>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rotor => "Rotor",
    versions: [Rotor => "Rotor"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rotor,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rotor,
            syntax: concat!(r#"""#, output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r#"""#, output_for_hello_world!()))],
        }],
    };
