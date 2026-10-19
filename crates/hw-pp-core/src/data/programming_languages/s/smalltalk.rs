//! Hello-world snippet for `ProgrammingLanguageName::Smalltalk`.
//!
//! Imported from leachim6/hello-world (`s/SmallTalk.sm` at `a152253`),
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
    language: Smalltalk => "SmallTalk",
    versions: [Smalltalk => "SmallTalk"],
    extensions: [Sm => "sm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smalltalk,
        extensions: &[FileExtension::Sm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smalltalk,
            syntax: concat!(r"Transcript show: '", output_for_hello_world!(), r"'."),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"Transcript show: '", output_for_hello_world!(), r"'."),
            )],
        }],
    };
