//! Hello-world snippet for `ProgrammingLanguageName::SelfN`.
//!
//! Imported from leachim6/hello-world (`s/Self.self` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Self.self>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SelfN,
            syntax: concat!(r"'", output_for_hello_world!(), r"' printLine"),
            tokenized_syntax: &[
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Plain, r" printLine"),
            ],
        }],
    };
