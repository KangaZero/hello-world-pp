//! Hello-world snippet for `ProgrammingLanguageName::SelfN`.
//!
//! Imported from leachim6/hello-world (`s/Self.self` at `a152253`),
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
    language: SelfN => "Self",
    versions: [SelfN => "Self"],
    extensions: [SelfE => "self"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SelfN,
        extensions: &[FileExtension::SelfE],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SelfN,
            syntax: concat!(r"'", output_for_hello_world!(), r"' printLine"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"'", output_for_hello_world!(), r"' printLine"),
            )],
        }],
    };
