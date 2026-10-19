//! Hello-world snippet for `ProgrammingLanguageName::Obj`.
//!
//! Imported from leachim6/hello-world (`o/Obj.st` at `a152253`),
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
    language: Obj => "Obj",
    versions: [Obj => "Obj"],
    extensions: [St => "st"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Obj,
        extensions: &[FileExtension::St],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Obj,
            syntax: concat!(r"stdout println:'", output_for_hello_world!(), r"'."),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"stdout println:'", output_for_hello_world!(), r"'."),
            )],
        }],
    };
