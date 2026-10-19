//! Hello-world snippet for `ProgrammingLanguageName::Move`.
//!
//! Imported from leachim6/hello-world (`m/Move.mv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Move => "Move",
    versions: [Move => "Move"],
    extensions: [Mv => "mv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Move,
        extensions: &[FileExtension::Mv],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Move,
            syntax: concat!(r"alert '", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"alert '", output_for_hello_world!(), r"'"),
            )],
        }],
    };
