//! Hello-world snippet for `ProgrammingLanguageName::Dncl`.
//!
//! Imported from leachim6/hello-world (`d/DNCL.dncl` at `a152253`),
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
    language: Dncl => "DNCL",
    versions: [Dncl => "DNCL"],
    extensions: [Dncl => "dncl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dncl,
        extensions: &[FileExtension::Dncl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dncl,
            syntax: concat!(r"「", output_for_hello_world!(), r"」を表示する "),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」を表示する "),
            )],
        }],
    };
