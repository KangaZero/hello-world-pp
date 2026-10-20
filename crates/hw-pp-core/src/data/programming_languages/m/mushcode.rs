//! Hello-world snippet for `ProgrammingLanguageName::Mushcode`.
//!
//! Imported from leachim6/hello-world (`m/Mushcode.mush` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mushcode.mush>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mushcode,
            syntax: concat!(r"say ", output_for_hello_world!(), r"."),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"say ", output_for_hello_world!(), r"."),
            )],
        }],
    };
