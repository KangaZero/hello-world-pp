//! Hello-world snippet for `ProgrammingLanguageName::Avisynth`.
//!
//! Imported from leachim6/hello-world (`a/Avisynth.avs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Avisynth.avs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Avisynth => "Avisynth",
    versions: [Avisynth => "Avisynth"],
    extensions: [Avs => "avs"],
}

const AVISYNTH: &str = concat!(r#"MessageClip(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Avisynth,
        extensions: &[FileExtension::Avs],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Avisynth,
            syntax: AVISYNTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AVISYNTH,
                    [(
                        TokenKind::Plain,
                        concat!(r#"MessageClip(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
