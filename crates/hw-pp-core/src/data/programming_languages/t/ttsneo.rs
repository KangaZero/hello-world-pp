//! Hello-world snippet for `ProgrammingLanguageName::Ttsneo`.
//!
//! Imported from leachim6/hello-world (`t/TTSneo.ttsneo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TTSneo.ttsneo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ttsneo => "TTSneo",
    versions: [Ttsneo => "TTSneo"],
    extensions: [Ttsneo => "ttsneo"],
}

const TTSNEO: &str = concat!(r"「", output_for_hello_world!(), r"」と出力する");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ttsneo,
        extensions: &[FileExtension::Ttsneo],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ttsneo,
            syntax: TTSNEO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TTSNEO,
                    [(
                        TokenKind::Plain,
                        concat!(r"「", output_for_hello_world!(), r"」と出力する"),
                    )],
                )
            },
        }],
    };
