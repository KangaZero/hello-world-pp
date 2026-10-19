//! Hello-world snippet for `ProgrammingLanguageName::U306aU3066U3057U3053`.
//!
//! Imported from leachim6/hello-world (`#/なでしこ.nako` at `a152253`),
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
    language: U306aU3066U3057U3053 => "なでしこ",
    versions: [U306aU3066U3057U3053 => "なでしこ"],
    extensions: [Nako => "nako"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U306aU3066U3057U3053,
        extensions: &[FileExtension::Nako],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U306aU3066U3057U3053,
            syntax: concat!(r"「", output_for_hello_world!(), r"」と表示"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」と表示"),
            )],
        }],
    };
