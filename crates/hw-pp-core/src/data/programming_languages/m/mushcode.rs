//! Hello-world snippet for `ProgrammingLanguageName::Mushcode`.
//!
//! Imported from leachim6/hello-world (`m/Mushcode.mush` at `a152253`),
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
    language: Mushcode => "Mushcode",
    versions: [Mushcode => "Mushcode"],
    extensions: [Mush => "mush"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mushcode,
        extensions: &[FileExtension::Mush],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mushcode,
            syntax: concat!(r"say ", output_for_hello_world!(), r"."),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"say ", output_for_hello_world!(), r"."),
            )],
        }],
    };
