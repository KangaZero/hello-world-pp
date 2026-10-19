//! Hello-world snippet for `ProgrammingLanguageName::Avisynth`.
//!
//! Imported from leachim6/hello-world (`a/Avisynth.avs` at `a152253`),
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
    language: Avisynth => "Avisynth",
    versions: [Avisynth => "Avisynth"],
    extensions: [Avs => "avs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Avisynth,
        extensions: &[FileExtension::Avs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Avisynth,
            syntax: concat!(r#"MessageClip(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"MessageClip(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
