//! Hello-world snippet for `ProgrammingLanguageName::Ttsneo`.
//!
//! Imported from leachim6/hello-world (`t/TTSneo.ttsneo` at `a152253`),
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
    language: Ttsneo => "TTSneo",
    versions: [Ttsneo => "TTSneo"],
    extensions: [Ttsneo => "ttsneo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ttsneo,
        extensions: &[FileExtension::Ttsneo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ttsneo,
            syntax: concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            )],
        }],
    };
