//! Hello-world snippet for `ProgrammingLanguageName::Emoticon`.
//!
//! Imported from leachim6/hello-world (`e/Emoticon.emoticon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emoticon.emoticon>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Emoticon => "Emoticon",
    versions: [Emoticon => "Emoticon"],
    extensions: [Emoticon => "emoticon"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emoticon,
        extensions: &[FileExtension::Emoticon],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emoticon,
            syntax: concat!(output_for_hello_world!(), r" :Q S::P :P"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(output_for_hello_world!(), r" :Q S::P :P"),
            )],
        }],
    };
